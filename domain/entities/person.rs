use chrono::NaiveDate;
use crate::entities::image::Image;

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct Person {
  pub id: i32,
  pub name: String,
  pub first_name: Option<String>,
  pub last_name: Option<String>,
  pub description: Option<String>,
  pub birthday: Option<NaiveDate>,
  pub height: Option<i16>,
  pub image: Option<Image>,
}
