use crate::entities::book::book_translation::BookTranslation;
use crate::entities::character::character_translation::CharacterTranslation;
use crate::entities::franchise::franchise_translation::FranchiseTranslation;
use crate::entities::genre::genre_translation::GenreTranslation;
use crate::entities::person::person_translation::PersonTranslation;
use crate::entities::role::role_translation::RoleTranslation;
use crate::entities::theme::theme_translation::ThemeTranslation;
use crate::enums::language::Language;
use std::collections::HashMap;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[cfg_attr(feature = "utoipa", aliases(
  GenreAvailableTranslations = AvailableTranslations < GenreTranslation >,
  ThemeAvailableTranslations = AvailableTranslations < ThemeTranslation >,
  CharacterAvailableTranslations = AvailableTranslations < CharacterTranslation >,
  RoleAvailableTranslations = AvailableTranslations < RoleTranslation >,
  PersonAvailableTranslations = AvailableTranslations < PersonTranslation >,
  FranchiseAvailableTranslations = AvailableTranslations < FranchiseTranslation >,
  BookAvailableTranslations = AvailableTranslations < BookTranslation >
))]
pub struct AvailableTranslations<T> {
  pub available_languages: Vec<Language>,
  pub translations: HashMap<Language, T>,
}
