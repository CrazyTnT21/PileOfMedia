use from_row::FromRow;
use tokio_postgres::Row;
use crate::enums::language::DbLanguage;

#[derive(FromRow, Debug)]
pub struct DbBookTranslation {
  pub title: String,
  pub description: Option<String>,
  #[rename = "fktranslation"]
  pub fk_translation: i32,
  pub language: DbLanguage,
}
