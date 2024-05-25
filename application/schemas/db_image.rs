use domain::entities::image::Image;
use from_row::FromRow;
use crate::enums::db_image_extension::DbImageExtension;
use tokio_postgres::Row;

#[derive(Debug, FromRow)]
#[rename = "image"]
pub struct DbImage {
  pub id: i32,
  pub uri: String,
  pub width: i16,
  pub height: i16,
  pub extension: Option<DbImageExtension>,
}

impl DbImage {
  pub fn to_entity(self) -> Image {
    Image {
      id: self.id,
      uri: self.uri,
      width: self.width,
      height: self.height,
      extension: self.extension.map(|x|x.into()),
    }
  }
}
