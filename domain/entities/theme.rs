pub mod create_partial_theme;
pub mod create_theme;
pub mod theme_translation;

use crate::available_translations::AvailableTranslations;
use crate::entities::theme::theme_translation::ThemeTranslation;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct Theme {
  pub id: u32,
  pub translations: ThemeAvailableTranslations,
}

type ThemeAvailableTranslations = AvailableTranslations<ThemeTranslation>;
