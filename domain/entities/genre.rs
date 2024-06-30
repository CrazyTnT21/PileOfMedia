pub mod create_genre;
pub mod create_partial_genre;

use crate::enums::language::Language;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct Genre {
  pub id: u32,
  pub name: String,
  pub language: Language,
}
