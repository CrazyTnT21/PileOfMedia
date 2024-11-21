use crate::entities::image::Image;
use crate::enums::language::Language;
use chrono::NaiveDate;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct Character {
  pub id: u32,
  pub name: String,
  pub first_name: Option<String>,
  pub last_name: Option<String>,
  pub description: Option<String>,
  pub birthday: Option<NaiveDate>,
  pub height: Option<u32>,
  pub image: Option<Image>,
  pub language: Language,
}
