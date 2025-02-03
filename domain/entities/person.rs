pub mod create_partial_person;
pub mod create_person;
pub mod person_role;
pub mod person_translation;

use crate::available_translations::AvailableTranslations;
use crate::entities::image::Image;
use crate::entities::person::person_translation::PersonTranslation;
use chrono::NaiveDate;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct Person {
  pub id: u32,
  pub name: String,
  pub first_name: Option<String>,
  pub last_name: Option<String>,
  pub birthday: Option<NaiveDate>,
  pub height_cm: Option<u16>,
  pub image: Option<Image>,
  pub translations: PersonAvailableTranslations,
}

type PersonAvailableTranslations = AvailableTranslations<PersonTranslation>;
