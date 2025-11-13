use from_row::FromRow;
use tokio_postgres::Row;

#[derive(FromRow, Debug)]
#[rename = "book_character"]
pub struct DbBookCharacter {
  pub book_id: i32,
  pub character_id: i32,
}
