use crate::entities::image::image_data::ImageData;

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct Image {
  pub id: i32,
  pub versions: Vec<ImageData>,
}
pub mod image_data;
pub mod partial_create_image;
pub mod create_image;

