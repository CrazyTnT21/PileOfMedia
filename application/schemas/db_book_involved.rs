use from_row::FromRow;
use tokio_postgres::Row;

#[derive(FromRow, Debug)]
#[rename = "book_involved"]
pub struct DbBookInvolved {
  pub book_id: i32,
  pub role_id: i32,
  pub person_id: i32,
}
