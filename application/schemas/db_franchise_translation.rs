use tokio_postgres::Row;

use domain::entities::franchise::franchise_translation::FranchiseTranslation;
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
impl DbFranchiseTranslation {
  pub fn to_entity(self) -> FranchiseTranslation {
    FranchiseTranslation { name: self.name }
  }
}
