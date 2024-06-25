use tokio_postgres::Row;
use from_row::FromRow;

use crate::enums::db_language::DbLanguage;

#[derive(FromRow, Debug)]
#[rename = "franchisetranslation"]
pub struct DbFranchiseTranslation {
  pub name: String,
  #[rename = "fktranslation"]
  pub fk_translation: i32,
  pub language: DbLanguage,
}
