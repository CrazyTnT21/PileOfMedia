use std::collections::HashMap;

use chrono::NaiveDate;
use crate::entities::image::create_image::CreateImage;

use crate::enums::language::Language;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreatePerson {
  pub name: String,
  pub first_name: Option<String>,
  pub last_name: Option<String>,
  pub birthday: Option<NaiveDate>,
  pub height: Option<u16>,
  pub image: Option<CreateImage>,
  pub translations: HashMap<Language, CreatePersonTranslation>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreatePersonTranslation {
  pub description: Option<String>,
}
