use tokio_postgres::Row;

use domain::entities::image::image_data::ImageData;
use from_row::FromRow;

#[derive(Debug, FromRow)]
#[rename = "imagedata"]
pub struct DbImageData {
  #[rename = "fkimage"]
  pub fk_image: i32,
  pub uri: String,
  pub width: i16,
  pub height: i16,
}

impl DbImageData {
  pub fn to_entity(self) -> ImageData {
    ImageData {
      uri: self.uri,
      width: self.width as u16,
      height: self.height as u16,
    }
  }
}
