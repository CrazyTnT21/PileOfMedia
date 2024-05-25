use std::error::Error;
use std::str::FromStr;

use crate::entities::image::ImageExtension::{GIF, JPEG, JPG, PNG};

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct Image {
  pub id: i32,
  pub uri: String,
  pub width: i16,
  pub height: i16,
  pub extension: Option<ImageExtension>,
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum ImageExtension {
  JPG,
  JPEG,
  PNG,
  GIF,
}

impl FromStr for ImageExtension {
  type Err = Box<dyn Error + Sync + Send>;

  fn from_str(value: &str) -> Result<Self, Self::Err> {
    match value.to_lowercase().as_str() {
      "jpg" => Ok(JPG),
      "jpeg" => Ok(JPEG),
      "png" => Ok(PNG),
      "gif" => Ok(GIF),
      _ => Err(Box::from(format!("Unknown image extension, {value}")))
    }
  }
}

