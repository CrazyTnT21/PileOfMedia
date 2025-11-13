use crate::enums::db_language::DbLanguage;
use domain::entities::book::book_translation::BookTranslation;
use domain::entities::image::Image;
use from_row::FromRow;
use tokio_postgres::Row;

#[derive(FromRow, Debug)]
#[rename = "book_translation"]
pub struct DbBookTranslation {
  pub title: String,
  pub description: Option<String>,
  pub cover_id: i32,
  pub translation_id: i32,
  pub language: DbLanguage,
}
impl DbBookTranslation {
  pub fn to_entity(self, cover: Image) -> BookTranslation {
    BookTranslation {
      title: self.title,
      description: self.description,
      cover,
    }
  }
}
