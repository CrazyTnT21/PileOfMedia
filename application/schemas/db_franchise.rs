use domain::available_translations::AvailableTranslations;
use domain::entities::franchise::franchise_translation::FranchiseTranslation;
use domain::entities::franchise::Franchise;
use from_row::FromRow;
use tokio_postgres::Row;

#[derive(FromRow, Debug)]
#[rename = "franchise"]
pub struct DbFranchise {
  pub id: i32,
}

impl DbFranchise {
  pub const fn to_entity(self, translations: AvailableTranslations<FranchiseTranslation>) -> Franchise {
    Franchise {
      id: self.id as u32,
      translations,
    }
  }
}
