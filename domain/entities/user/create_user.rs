use crate::entities::image::Image;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreateUser {
  pub name: String,
  pub description: Option<String>,
  pub profile_picture: Option<Image>,
}
