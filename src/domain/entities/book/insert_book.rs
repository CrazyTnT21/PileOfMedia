use std::collections::HashMap;
use serde::Serialize;
use chrono::NaiveDate;
use crate::domain::entities::franchise::franchise::Franchise;
use crate::domain::enums::language::Language;

#[derive(Serialize)]
pub struct InsertBook {
  pub chapters: Option<i16>,
  pub pages: Option<i16>,
  pub words: Option<i32>,
  pub published: Option<NaiveDate>,
  pub franchise: Option<Franchise>,
  pub translations: HashMap<Language, TitleDescription>,
}

#[derive(Serialize)]
pub struct TitleDescription {
  title: String,
  description: Option<String>,
}
