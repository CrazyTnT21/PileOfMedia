use std::collections::HashMap;

use chrono::NaiveDate;

use crate::entities::image::Image;
use crate::enums::language::Language;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreatePartialCharacter {
  pub birthday: Option<NaiveDate>,
  pub height_cm: Option<u32>,
  pub image: Option<Image>,
  pub translations: HashMap<Language, CreatePartialCharacterTranslation>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreatePartialCharacterTranslation {
  pub name: String,
  pub first_name: Option<String>,
  pub last_name: Option<String>,
  pub description: Option<String>,
}
