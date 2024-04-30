use tokio_postgres::Row;
use domain::entities::franchise::franchise::Franchise;
use from_row::FromRow;

#[derive(FromRow, Debug)]
pub struct DbFranchise {
  pub id: i32,
  pub name: String,
}

impl DbFranchise {
  pub fn to_entity(self) -> Franchise {
    Franchise {
      id: self.id,
      name: self.name,
    }
  }
}
