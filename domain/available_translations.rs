use crate::entities::character::character_translation::CharacterTranslation;
use crate::entities::genre::genre_translation::GenreTranslation;
use crate::entities::theme::theme_translation::ThemeTranslation;
use crate::enums::language::Language;
use std::collections::HashMap;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[cfg_attr(feature = "utoipa", aliases(
  GenreAvailableTranslations = AvailableTranslations < GenreTranslation >,
  ThemeAvailableTranslations = AvailableTranslations < ThemeTranslation >,
  CharacterAvailableTranslations = AvailableTranslations < CharacterTranslation >
))]
pub struct AvailableTranslations<T> {
  pub available_languages: Vec<Language>,
  pub translations: HashMap<Language, T>,
}
