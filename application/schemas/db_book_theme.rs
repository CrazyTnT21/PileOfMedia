use from_row::FromRow;
use tokio_postgres::Row;

#[derive(FromRow, Debug)]
#[rename = "booktheme"]
pub struct DbBookTheme {
  #[rename = "fkbook"]
  pub fk_book: i32,
  #[rename = "fktheme"]
  pub fk_theme: i32,
}
