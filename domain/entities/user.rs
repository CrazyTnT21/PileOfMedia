pub mod create_user;
pub mod partial_create_user;

use chrono::NaiveDate;
use crate::entities::image::Image;

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct User {
  pub id: i32,
  pub name: String,
  pub joined: NaiveDate,
  pub description: Option<String>,
  pub deleted: bool,
  pub profile_picture: Option<Image>
}
