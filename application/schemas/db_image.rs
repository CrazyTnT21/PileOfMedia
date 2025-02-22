use domain::entities::image::Image;
use domain::entities::image::image_data::ImageData;
use from_row::FromRow;
use tokio_postgres::Row;

#[derive(Debug, FromRow)]
#[rename = "image"]
pub struct DbImage {
  pub id: i32,
}

impl DbImage {
  pub const fn to_entity(self, versions: Vec<ImageData>) -> Image {
    Image {
      id: self.id as u32,
      versions,
    }
  }
}
