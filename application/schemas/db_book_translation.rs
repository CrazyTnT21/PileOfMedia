use crate::enums::db_language::DbLanguage;
use domain::entities::book::book_translation::BookTranslation;
use domain::entities::image::Image;
use from_row::FromRow;
use tokio_postgres::Row;

#[derive(FromRow, Debug)]
#[rename = "booktranslation"]
pub struct DbBookTranslation {
  pub title: String,
  pub description: Option<String>,
  #[rename = "fkcover"]
  pub fk_cover: i32,
  #[rename = "fktranslation"]
  pub fk_translation: i32,
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
