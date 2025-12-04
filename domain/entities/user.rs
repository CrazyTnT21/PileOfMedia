pub mod create_partial_user;
pub mod create_user;
pub mod create_user_book;
pub mod user_book;
pub mod user_status;

use crate::entities::image::Image;
use chrono::NaiveDate;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct User {
  pub id: u32,
  pub name: String,
  pub joined: NaiveDate,
  pub description: Option<String>,
  pub profile_picture: Option<Image>,
}
