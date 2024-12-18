use crate::entities::rating::Rating;
use chrono::NaiveDate;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct BookStatistic {
  pub rating: Rating,
  pub rank: u32,
  pub popularity: u32,
  pub favorites: u32,
  pub members: u32,
  pub added: NaiveDate,
}
