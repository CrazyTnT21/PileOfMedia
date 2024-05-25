use tokio_postgres::Row;
use from_row::FromRow;

#[derive(FromRow, Debug)]
#[rename = "booktheme"]
pub struct DbBookTheme {
  #[rename = "fkbook"]
  pub fk_book: i32,
  #[rename = "fktheme"]
  pub fk_theme: i32,
}
