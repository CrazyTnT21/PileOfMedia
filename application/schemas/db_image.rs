use tokio_postgres::Row;
use domain::entities::image::image::Image;
use from_row::FromRow;
use crate::schemas::db_image_translation::DbImageTranslation;

#[derive(Debug, FromRow)]
#[rename = "image"]
pub struct DbImage {
  pub id: i32,
}

impl DbImage {
  pub fn to_entity(self, image_translation: DbImageTranslation) -> Image {
    Image {
      id: self.id,
      uri: image_translation.uri,
      width: image_translation.width,
      height: image_translation.height,
      extension: image_translation.extension.and_then(|x| Some(x.into())),
    }
  }
}
