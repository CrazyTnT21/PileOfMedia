pub mod create_franchise;
pub mod create_partial_franchise;
pub mod franchise_translation;

use crate::available_translations::AvailableTranslations;
use crate::entities::franchise::franchise_translation::FranchiseTranslation;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct Franchise {
  pub id: u32,
  pub translations: FranchiseAvailableTranslations,
}

type FranchiseAvailableTranslations = AvailableTranslations<FranchiseTranslation>;
