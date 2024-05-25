use tokio_postgres::Row;
use from_row::FromRow;

#[derive(FromRow, Debug)]
#[rename = "bookgenre"]
pub struct DbBookGenre {
  #[rename = "fkbook"]
  pub fk_book: i32,
  #[rename = "fkgenre"]
  pub fk_genre: i32,
}
