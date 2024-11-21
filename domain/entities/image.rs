use crate::entities::image::image_data::ImageData;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct Image {
  pub id: u32,
  pub versions: Vec<ImageData>,
}
pub mod create_image;
pub mod create_partial_image;
pub mod image_data;
