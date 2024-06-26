pub mod create_user;
pub mod create_partial_user;

use chrono::NaiveDate;
use crate::entities::image::Image;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct User {
  pub id: u32,
  pub name: String,
  pub joined: NaiveDate,
  pub description: Option<String>,
  pub deleted: bool,
  pub profile_picture: Option<Image>,
}
