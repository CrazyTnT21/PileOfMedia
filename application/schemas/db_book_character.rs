use tokio_postgres::Row;
use from_row::FromRow;

#[derive(FromRow, Debug)]
#[rename = "bookcharacter"]
pub struct DbBookCharacter {
  #[rename = "fkbook"]
  pub fk_book: i32,
  #[rename = "fkcharacter"]
  pub fk_character: i32,
}
