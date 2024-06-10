use crate::entities::image::partial_create_image::PartialCreateImage;

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct PartialCreateUser {
  pub name: String,
  pub description: Option<String>,
  pub profile_picture: Option<PartialCreateImage>,
}
