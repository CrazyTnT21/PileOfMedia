use crate::entities::genre::genre_translation::GenreTranslation;
use crate::enums::language::Language;
use std::collections::HashMap;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[cfg_attr(feature = "utoipa", aliases(
  GenreAvailableTranslations = AvailableTranslations < GenreTranslation >
))]
pub struct AvailableTranslations<T> {
  pub languages: Vec<Language>,
  pub translations: HashMap<Language, T>,
}
