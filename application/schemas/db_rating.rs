use tokio_postgres::Row;

use domain::entities::rating::Rating;
use from_row::FromRow;

#[derive(FromRow, Debug)]
#[rename = "rating"]
pub struct DbRating {
  id: i32,
  score: Option<f32>,
  amount: i32,
}

impl DbRating {
  pub const fn to_entity(self) -> Rating {
    Rating {
      id: self.id as u32,
      score: self.score,
      amount: self.amount as u32,
    }
  }
}
