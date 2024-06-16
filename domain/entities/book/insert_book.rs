use std::collections::HashMap;
use chrono::NaiveDate;
use crate::entities::franchise::Franchise;
use crate::enums::language::Language;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct InsertBook {
  pub chapters: Option<u16>,
  pub pages: Option<u16>,
  pub words: Option<u32>,
  pub published: Option<NaiveDate>,
  pub franchise: Option<Franchise>,
  pub translations: HashMap<Language, TitleDescription>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct TitleDescription {
  title: String,
  description: Option<String>,
}
