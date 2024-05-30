use std::error::Error;

use async_trait::async_trait;
use tokio_postgres::types::ToSql;

use domain::entities::genre::Genre;
use domain::enums::language::Language;
use domain::items_total::ItemsTotal;
use domain::pagination::Pagination;
use repositories::genre_repository::GenreRepository;
use crate::enums::db_language::DbLanguage;
use crate::fallback_unwrap::fallback_unwrap;

use crate::Pooled;
use crate::schemas::db_genre::DbGenre;
use crate::schemas::db_genre_translation::DbGenreTranslation;
use crate::select::comparison::Comparison::{Equal, ILike, In};
use crate::select::condition::Condition::{Column, Value};
use crate::select::expression::Expression;
use crate::select::Select;

pub struct DefaultGenreRepository<'a> {
  pool: &'a Pooled<'a>,
  default_language: DbLanguage,
}

impl<'a> DefaultGenreRepository<'a> {
  pub fn new(pool: &'a Pooled, language: Language) -> DefaultGenreRepository<'a> {
    DefaultGenreRepository { pool, default_language: language.into() }
  }
}

#[async_trait]
impl<'a> GenreRepository for DefaultGenreRepository<'a> {
  async fn get(&self, language: Language, pagination: Pagination) -> Result<ItemsTotal<Genre>, Box<dyn Error>> {
    let language = DbLanguage::from(language);
    let select = Select::new("genre")
      .columns::<DbGenre>("genre")
      .columns::<Option<DbGenreTranslation>>("genre_translation")
      .columns::<Option<DbGenreTranslation>>("genre_translation_fallback")
      .left_join("genretranslation", Some("genre_translation"),
                 Expression::column_equal("genre_translation", "language", &language)
                   .and(Expression::new(Column(("genre_translation", "fktranslation"), ("genre", "id")))),
      )
      .left_join("genretranslation", Some("genre_translation_fallback"),
                 Expression::column_equal("genre_translation_fallback", "language", &self.default_language)
                   .and(Expression::new(Column(("genre_translation_fallback", "fktranslation"), ("genre", "id"))))
                   .and(Expression::column_null("genre_translation", "fktranslation")),
      );

    let total = select.count(self.pool).await? as usize;

    let genres = select
      .pagination(pagination)
      .query(self.pool)
      .await?;

    let genres = to_entities(genres);
    Ok(ItemsTotal { items: genres, total })
  }

  async fn get_by_id(&self, id: u32, language: Language) -> Result<Option<Genre>, Box<dyn Error>> {
    let id = id as i32;
    let language = DbLanguage::from(language);
    let genre = Select::new("genre")
      .columns::<DbGenre>("genre")
      .columns::<Option<DbGenreTranslation>>("genre_translation")
      .columns::<Option<DbGenreTranslation>>("genre_translation_fallback")
      .left_join("genretranslation", Some("genre_translation"),
                 Expression::column_equal("genre_translation", "language", &language)
                   .and(Expression::new(Column(("genre_translation", "fktranslation"), ("genre", "id")))),
      )
      .left_join("genretranslation", Some("genre_translation_fallback"),
                 Expression::column_equal("genre_translation_fallback", "language", &self.default_language)
                   .and(Expression::new(Column(("genre_translation_fallback", "fktranslation"), ("genre", "id"))))
                   .and(Expression::column_null("genre_translation", "fktranslation")),
      )
      .where_expression(Expression::new(Value(("genre", "id"), Equal(&id))))
      .get_single(self.pool)
      .await?;

    Ok(genre.map(to_entity))
  }

  async fn get_by_ids(&self, ids: &[i32], language: Language) -> Result<Vec<Genre>, Box<dyn Error>> {
    let language = DbLanguage::from(language);
    let ids = convert(ids);
    let genres = Select::new("genre")
      .columns::<DbGenre>("genre")
      .columns::<Option<DbGenreTranslation>>("genre_translation")
      .columns::<Option<DbGenreTranslation>>("genre_translation_fallback")
      .where_expression(Expression::new(Value(("genre", "id"), In(&ids))))
      .left_join("genretranslation", Some("genre_translation"),
                 Expression::column_equal("genre_translation", "language", &language)
                   .and(Expression::new(Column(("genre_translation", "fktranslation"), ("genre", "id")))),
      )
      .left_join("genretranslation", Some("genre_translation_fallback"),
                 Expression::column_equal("genre_translation_fallback", "language", &self.default_language)
                   .and(Expression::new(Column(("genre_translation_fallback", "fktranslation"), ("genre", "id"))))
                   .and(Expression::column_null("genre_translation", "fktranslation")),
      )
      .query(self.pool)
      .await?;

    Ok(to_entities(genres))
  }

  async fn get_by_name(&self, name: &str, language: Language, pagination: Pagination) -> Result<ItemsTotal<Genre>, Box<dyn Error>> {
    let language = DbLanguage::from(language);
    let name = format!("%{name}%");
    let select = Select::new("genre")
      .columns::<DbGenre>("genre")
      .columns::<Option<DbGenreTranslation>>("genre_translation")
      .columns::<Option<DbGenreTranslation>>("genre_translation_fallback")
      .left_join("genretranslation", Some("genre_translation"),
                 Expression::column_equal("genre_translation", "language", &language)
                   .and(Expression::new(Column(("genre_translation", "fktranslation"), ("genre", "id")))),
      )
      .left_join("genretranslation", Some("genre_translation_fallback"),
                 Expression::column_equal("genre_translation_fallback", "language", &self.default_language)
                   .and(Expression::new(Column(("genre_translation_fallback", "fktranslation"), ("genre", "id"))))
                   .and(Expression::column_null("genre_translation", "fktranslation")),
      )
      .where_expression(Expression::new(Value(("genre_translation", "name"), ILike(&name)))
        .or(Expression::new(Value(("genre_translation_fallback", "name"), ILike(&name)))));

    let total = select.count(self.pool).await? as usize;

    let genres = select
      .pagination(pagination)
      .query(self.pool)
      .await?
      .into_iter()
      .map(to_entity)
      .collect();
    Ok(ItemsTotal { items: genres, total })
  }
}

fn to_entities(genres: Vec<(DbGenre, Option<DbGenreTranslation>, Option<DbGenreTranslation>)>) -> Vec<Genre> {
  genres
    .into_iter()
    .map(to_entity)
    .collect()
}

fn to_entity(genre: (DbGenre, Option<DbGenreTranslation>, Option<DbGenreTranslation>)) -> Genre {
  genre.0.to_entity(fallback_unwrap(genre.1, genre.2))
}

fn convert(value: &[impl ToSql + Sync]) -> Vec<&(dyn ToSql + Sync)> {
  value.iter().map(|x| x as &(dyn ToSql + Sync)).collect::<Vec<&(dyn ToSql + Sync)>>()
}
