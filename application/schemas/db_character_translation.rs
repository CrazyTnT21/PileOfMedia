use tokio_postgres::Row;
use from_row::FromRow;

use crate::enums::db_language::DbLanguage;

#[derive(FromRow, Debug)]
#[rename = "charactertranslation"]
pub struct DbCharacterTranslation {
  pub name: String,
  #[rename = "firstname"]
  pub first_name: Option<String>,
  #[rename = "lastname"]
  pub last_name: Option<String>,
  pub description: Option<String>,
  #[rename = "fktranslation"]
  pub fk_translation: i32,
  pub language: DbLanguage,
}
