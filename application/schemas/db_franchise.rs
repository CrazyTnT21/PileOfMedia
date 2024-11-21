use crate::schemas::db_franchise_translation::DbFranchiseTranslation;
use domain::entities::franchise::Franchise;
use from_row::FromRow;
use tokio_postgres::Row;

#[derive(FromRow, Debug)]
#[rename = "franchise"]
pub struct DbFranchise {
  pub id: i32,
}

impl DbFranchise {
  pub fn to_entity(self, db_franchise_translation: DbFranchiseTranslation) -> Franchise {
    Franchise {
      id: self.id as u32,
      name: db_franchise_translation.name,
      language: db_franchise_translation.language.into(),
    }
  }
}
