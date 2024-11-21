use crate::enums::db_language::DbLanguage;
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
