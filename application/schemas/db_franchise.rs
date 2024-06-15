use tokio_postgres::Row;
use domain::entities::franchise::Franchise;
use from_row::FromRow;

#[derive(FromRow, Debug)]
#[rename = "franchise"]
pub struct DbFranchise {
  pub id: i32,
  pub name: String,
}

impl DbFranchise {
  pub fn to_entity(self) -> Franchise {
    Franchise {
      id: self.id as u32,
      name: self.name,
    }
  }
}
