use from_row::FromRow;
use tokio_postgres::Row;

use crate::enums::db_language::DbLanguage;

#[derive(FromRow, Debug)]
#[rename = "persontranslation"]
pub struct DbPersonTranslation {
  pub description: Option<String>,
  #[rename = "fktranslation"]
  pub fk_translation: i32,
  pub language: DbLanguage,
}
