use std::error::Error;

use async_trait::async_trait;
use tokio_postgres::Client;

use domain::entities::genre::Genre;
use domain::enums::language::Language;
use domain::items_total::ItemsTotal;
use domain::pagination::Pagination;
use from_row::{FromRow, Table};
use repositories::genre_repository::GenreRepository;

use crate::convert_to_sql::{to_i32};
use crate::enums::db_language::DbLanguage;
use crate::fallback_unwrap::fallback_unwrap;
use crate::schemas::db_genre::DbGenre;
use crate::schemas::db_genre_translation::DbGenreTranslation;
use crate::select::combined_tuple::CombinedType;
use crate::select::conditions::column_equal::ColumnEqual;
use crate::select::conditions::column_null::ColumnNull;
use crate::select::conditions::value_ilike::ValueILike;
use crate::select::conditions::value_equal::ValueEqual;
use crate::select::conditions::value_in::ValueIn;
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

    let total = Select::new::<DbGenre>()
      .transform(|x| self.genre_joins(x, &language))
      .count()
      .get_single(self.client)
      .await?
      .expect("Count should return one row");
    let total = total.0 as usize;

    let genres = genre_select_columns()
      .transform(|x| self.genre_joins(x, &language))
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
      .where_expression(Expression::new(ValueEqual::new(("genre", "id"), id)))
      .get_single(self.client)
      .await?;
    Ok(genre.map(to_entity))
  }

  async fn get_by_ids(&self, ids: &[u32], language: Language) -> Result<Vec<Genre>, Box<dyn Error>> {
    let language = DbLanguage::from(language);
    let ids = to_i32(ids);

    let genres = genre_select_columns()
      .transform(|x| self.genre_joins(x, &language))
      .where_expression(Expression::new(ValueIn::new(("genre", "id"), &ids)))
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

    let total = Select::new::<DbGenre>()
      .transform(|x| self.genre_joins(x, &language))
      .where_expression(Expression::new(ValueILike::new(("genre_translation", "name"), &name))
        .or(Expression::new(ValueILike::new(("genre_translation_fallback", "name"), &name))))
      .count()
      .get_single(self.client)
      .await?
      .expect("Count should return one row");

    let total = total.0 as usize;
    let genres = genre_select_columns()
      .transform(|x| self.genre_joins(x, &language))
      .where_expression(Expression::new(ValueILike::new(("genre_translation", "name"), &name))
        .or(Expression::new(ValueILike::new(("genre_translation_fallback", "name"), &name))))
      .pagination(pagination)
      .query(self.client)
      .await?
      .into_iter()
      .map(to_entity)
      .collect();
    Ok(ItemsTotal { items: genres, total })
  }

  async fn filter_existing(&self, genres: &[u32]) -> Result<Vec<u32>, Box<dyn Error>> {
    let genres = to_i32(genres);

    let count = Select::new::<DbGenre>()
      .column::<i32>(DbGenre::TABLE_NAME, "id")
      .where_expression(Expression::new(ValueIn::new((DbGenre::TABLE_NAME, "id"), &genres)))
      .query(self.client)
      .await?
      .into_iter()
      .map(|x| { x.0 as u32 })
      .collect();
    Ok(count)
  }
}

impl<'a> DefaultGenreRepository<'a> {
  fn genre_joins<T: FromRow<DbType=T> + CombinedType>(&'a self, select: Select<'a, T>, language: &'a DbLanguage) -> Select<'a, T> {
    select
      .left_join::<DbGenreTranslation>(
        Some("genre_translation"),
        Expression::column_equal("genre_translation", "language", language)
          .and(Expression::new(ColumnEqual::new(("genre_translation", "fktranslation"), ("genre", "id")))),
      )
      .left_join::<DbGenreTranslation>(
        Some("genre_translation_fallback"),
        Expression::column_equal("genre_translation_fallback", "language", &self.default_language)
          .and(Expression::new(ColumnEqual::new(("genre_translation_fallback", "fktranslation"), ("genre", "id"))))
          .and(Expression::new(ColumnNull::new(("genre_translation", "fktranslation")))),
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
