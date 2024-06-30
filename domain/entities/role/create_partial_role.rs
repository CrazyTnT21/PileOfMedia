use std::collections::HashMap;

use crate::enums::language::Language;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreatePartialRole {
  pub translations: HashMap<Language, CreatePartialRoleTranslation>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreatePartialRoleTranslation {
  pub name: String,
}
