use from_row::FromRow;
use tokio_postgres::Row;

#[derive(FromRow, Debug)]
#[rename = "book_theme"]
pub struct DbBookTheme {
  pub book_id: i32,
  pub theme_id: i32,
}
