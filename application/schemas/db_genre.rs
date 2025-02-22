use domain::available_translations::AvailableTranslations;
use domain::entities::genre::Genre;
use domain::entities::genre::genre_translation::GenreTranslation;
use from_row::FromRow;
use tokio_postgres::Row;

#[derive(FromRow, Debug)]
#[rename = "genre"]
pub struct DbGenre {
  pub id: i32,
}

impl DbGenre {
  pub const fn to_entity(self, translations: AvailableTranslations<GenreTranslation>) -> Genre {
    Genre {
      id: self.id as u32,
      translations,
    }
  }
}
