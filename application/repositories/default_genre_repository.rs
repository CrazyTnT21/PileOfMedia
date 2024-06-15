use std::error::Error;

use async_trait::async_trait;
use tokio_postgres::Client;

use domain::entities::genre::Genre;
use domain::enums::language::Language;
use domain::items_total::ItemsTotal;
use domain::pagination::Pagination;
use from_row::FromRow;
use repositories::genre_repository::GenreRepository;

use crate::convert_to_sql::{convert_to_sql, to_i32};
use crate::enums::db_language::DbLanguage;
use crate::fallback_unwrap::fallback_unwrap;
use crate::schemas::db_genre::DbGenre;
use crate::schemas::db_genre_translation::DbGenreTranslation;
use crate::select::combined_tuple::CombinedType;
use crate::select::comparison::Comparison::{Equal, ILike, In};
use crate::select::condition::Condition::{Column, Value};
use crate::select::expression::Expression;
use crate::select::Select;

pub struct DefaultGenreRepository<'a> {
  client: &'a Client,
  default_language: DbLanguage,
}

impl<'a> DefaultGenreRepository<'a> {
  pub fn new(client: &'a Client, language: Language) -> DefaultGenreRepository<'a> {
    DefaultGenreRepository { client, default_language: language.into() }
  }
}

#[async_trait]
impl<'a> GenreRepository for DefaultGenreRepository<'a> {
  async fn get(&self, language: Language, pagination: Pagination) -> Result<ItemsTotal<Genre>, Box<dyn Error>> {
    let language = DbLanguage::from(language);
    let select = genre_select_columns()
      .transform(|x| self.genre_joins(x, &language));

    let total = select.count(self.client).await? as usize;

    let genres = select
      .pagination(pagination)
      .query(self.client)
      .await?
      .into_iter()
      .map(to_entity)
      .collect();

    Ok(ItemsTotal { items: genres, total })
  }

  async fn get_by_id(&self, id: u32, language: Language) -> Result<Option<Genre>, Box<dyn Error>> {
    let id = id as i32;
    let language = DbLanguage::from(language);
    let genre = genre_select_columns()
      .transform(|x| self.genre_joins(x, &language))
      .where_expression(Expression::new(Value(("genre", "id"), Equal(&id))))
      .get_single(self.client)
      .await?;

    Ok(genre.map(to_entity))
  }

  async fn get_by_ids(&self, ids: &[u32], language: Language) -> Result<Vec<Genre>, Box<dyn Error>> {
    let language = DbLanguage::from(language);
    let ids = to_i32(ids);
    let ids = convert_to_sql(&ids);
    let genres = genre_select_columns()
      .transform(|x| self.genre_joins(x, &language))
      .where_expression(Expression::new(Value(("genre", "id"), In(&ids))))
      .query(self.client)
      .await?
      .into_iter()
      .map(to_entity)
      .collect();

    Ok(genres)
  }

  async fn get_by_name(&self, name: &str, language: Language, pagination: Pagination) -> Result<ItemsTotal<Genre>, Box<dyn Error>> {
    let language = DbLanguage::from(language);
    let name = format!("%{name}%");
    let select = genre_select_columns()
      .transform(|x| self.genre_joins(x, &language))
      .where_expression(Expression::new(Value(("genre_translation", "name"), ILike(&name)))
        .or(Expression::new(Value(("genre_translation_fallback", "name"), ILike(&name)))));

    let total = select.count(self.client).await? as usize;

    let genres = select
      .pagination(pagination)
      .query(self.client)
      .await?
      .into_iter()
      .map(to_entity)
      .collect();
    Ok(ItemsTotal { items: genres, total })
  }
}

impl<'a> DefaultGenreRepository<'a> {
  fn genre_joins<T: FromRow<DbType=T> + CombinedType>(&'a self, select: Select<'a, T>, language: &'a DbLanguage) -> Select<'a, T> {
    select
      .left_join::<DbGenreTranslation>(
        Some("genre_translation"),
        Expression::column_equal("genre_translation", "language", language)
          .and(Expression::new(Column(("genre_translation", "fktranslation"), ("genre", "id")))),
      )
      .left_join::<DbGenreTranslation>(
        Some("genre_translation_fallback"),
        Expression::column_equal("genre_translation_fallback", "language", &self.default_language)
          .and(Expression::new(Column(("genre_translation_fallback", "fktranslation"), ("genre", "id"))))
          .and(Expression::column_null("genre_translation", "fktranslation")),
      )
  }
}

fn to_entity(genre: (DbGenre, Option<DbGenreTranslation>, Option<DbGenreTranslation>)) -> Genre {
  genre.0.to_entity(fallback_unwrap(genre.1, genre.2))
}

fn genre_select_columns<'a>() -> Select<'a, GenreColumns> {
  Select::new::<DbGenre>()
    .columns::<DbGenre>("genre")
    .columns::<Option<DbGenreTranslation>>("genre_translation")
    .columns::<Option<DbGenreTranslation>>("genre_translation_fallback")
}

type GenreColumns = (DbGenre, Option<DbGenreTranslation>, Option<DbGenreTranslation>);
