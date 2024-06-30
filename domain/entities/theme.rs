pub mod create_theme;
pub mod create_partial_theme;

use crate::enums::language::Language;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct Theme {
  pub id: u32,
  pub name: String,
  pub language: Language,
}
