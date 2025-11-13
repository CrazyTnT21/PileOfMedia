use tokio_postgres::Row;

use domain::entities::franchise::franchise_translation::FranchiseTranslation;
use from_row::FromRow;

use crate::enums::db_language::DbLanguage;

#[derive(FromRow, Debug)]
#[rename = "franchise_translation"]
pub struct DbFranchiseTranslation {
  pub name: String,
  pub translation_id: i32,
  pub language: DbLanguage,
}
impl DbFranchiseTranslation {
  pub fn to_entity(self) -> FranchiseTranslation {
    FranchiseTranslation { name: self.name }
  }
}
