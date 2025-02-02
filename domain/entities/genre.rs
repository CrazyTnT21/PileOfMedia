pub mod create_genre;
pub mod create_partial_genre;
pub mod genre_translation;

use crate::available_translations::AvailableTranslations;
use crate::entities::genre::genre_translation::GenreTranslation;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct Genre {
  pub id: u32,
  pub translations: GenreAvailableTranslations,
}

type GenreAvailableTranslations = AvailableTranslations<GenreTranslation>;
