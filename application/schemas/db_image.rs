use domain::entities::image::Image;
use from_row::FromRow;
use tokio_postgres::Row;
use domain::entities::image::image_data::ImageData;

#[derive(Debug, FromRow)]
#[rename = "image"]
pub struct DbImage {
  pub id: i32,
}

impl DbImage {
  pub fn to_entity(self, versions: Vec<ImageData>) -> Image {
    Image {
      id: self.id,
      versions
    }
  }
}
