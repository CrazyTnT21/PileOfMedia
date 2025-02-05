use crate::entities::image::Image;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct BookTranslation {
  pub title: String,
  pub description: Option<String>,
  pub cover: Image,
}
