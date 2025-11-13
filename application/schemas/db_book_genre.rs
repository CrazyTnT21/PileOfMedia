use from_row::FromRow;
use tokio_postgres::Row;

#[derive(FromRow, Debug)]
#[rename = "book_genre"]
pub struct DbBookGenre {
  pub book_id: i32,
  pub genre_id: i32,
}
