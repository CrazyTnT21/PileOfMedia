use from_row::FromRow;
use tokio_postgres::Row;

#[derive(FromRow, Debug)]
#[rename = "bookcharacter"]
pub struct DbBookCharacter {
  #[rename = "fkbook"]
  pub fk_book: i32,
  #[rename = "fkcharacter"]
  pub fk_character: i32,
}
