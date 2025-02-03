pub mod create_character;
pub mod create_partial_character;

pub mod character_translation;

use crate::available_translations::AvailableTranslations;
use crate::entities::character::character_translation::CharacterTranslation;
use crate::entities::image::Image;
use chrono::NaiveDate;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct Character {
  pub id: u32,
  pub birthday: Option<NaiveDate>,
  pub height_cm: Option<u32>,
  pub image: Option<Image>,
  pub translations: CharacterAvailableTranslations,
}

type CharacterAvailableTranslations = AvailableTranslations<CharacterTranslation>;
