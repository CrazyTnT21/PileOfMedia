use from_row::FromRow;
use tokio_postgres::Row;

#[derive(FromRow, Debug)]
#[rename = "bookgenre"]
pub struct DbBookGenre {
  #[rename = "fkbook"]
  pub fk_book: i32,
  #[rename = "fkgenre"]
  pub fk_genre: i32,
}
