use serde::Serialize;

#[derive(Serialize)]
pub struct Image {
  pub id: i32,
  pub uri: String,
  pub width: i16,
  pub height: i16,
  pub extension: ImageExtension,
}

#[derive(Serialize)]
pub enum ImageExtension {
  JPG,
  PNG,
  GIF,
}
