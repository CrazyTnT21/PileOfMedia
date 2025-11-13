use crate::enums::db_language::DbLanguage;
use domain::entities::genre::genre_translation::GenreTranslation;
use from_row::FromRow;
use tokio_postgres::Row;

#[derive(FromRow, Debug)]
#[rename = "genre_translation"]
pub struct DbGenreTranslation {
  pub name: String,
  pub translation_id: i32,
  pub language: DbLanguage,
}
impl DbGenreTranslation {
  pub fn to_entity(self) -> GenreTranslation {
    GenreTranslation { name: self.name }
  }
}
