use std::error::Error;
use std::str::FromStr;

use serde::Serialize;
use crate::entities::image::image::ImageExtension::{GIF, JPEG, JPG, PNG};


#[derive(Serialize, Debug)]
pub struct Image {
  pub id: i32,
  pub uri: String,
  pub width: i16,
  pub height: i16,
  pub extension: Option<ImageExtension>,
}

#[derive(Serialize, Debug)]
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

