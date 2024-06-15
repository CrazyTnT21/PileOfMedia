use tokio_postgres::Row;

use domain::entities::genre::Genre;
use from_row::FromRow;

use crate::schemas::db_genre_translation::DbGenreTranslation;

#[derive(FromRow, Debug)]
#[rename = "genre"]
pub struct DbGenre {
  pub id: i32
}

impl DbGenre {
  pub fn to_entity(self, genre_translation: DbGenreTranslation) -> Genre {
   Genre {
     id: self.id as u32,
     name: genre_translation.name
   }
  }
}
