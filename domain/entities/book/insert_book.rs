use std::collections::HashMap;
use chrono::NaiveDate;
use crate::entities::franchise::franchise::Franchise;
use crate::enums::language::Language;

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct InsertBook {
  pub chapters: Option<i16>,
  pub pages: Option<i16>,
  pub words: Option<i32>,
  pub published: Option<NaiveDate>,
  pub franchise: Option<Franchise>,
  pub translations: HashMap<Language, TitleDescription>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct TitleDescription {
  title: String,
  description: Option<String>,
}
