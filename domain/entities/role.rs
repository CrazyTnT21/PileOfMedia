pub mod create_partial_role;
pub mod create_role;
pub mod role_translation;

use crate::available_translations::AvailableTranslations;
use crate::entities::role::role_translation::RoleTranslation;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct Role {
  pub id: u32,
  pub translations: RoleAvailableTranslations,
}

type RoleAvailableTranslations = AvailableTranslations<RoleTranslation>;
