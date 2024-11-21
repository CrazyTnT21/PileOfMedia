use crate::entities::book::book_involved::InvolvedId;
use crate::entities::image::Image;
use crate::enums::language::Language;
use chrono::NaiveDate;
use std::collections::HashMap;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreatePartialBook {
  pub chapters: Option<u16>,
  pub pages: Option<u16>,
  pub words: Option<u32>,
  pub published: Option<NaiveDate>,
  pub franchise: Option<u32>,
  pub translations: HashMap<Language, CreatePartialBookTranslation>,
  pub genres: Vec<u32>,
  pub themes: Vec<u32>,
  pub characters: Vec<u32>,
  pub involved: Vec<InvolvedId>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreatePartialBookTranslation {
  pub title: String,
  pub description: Option<String>,
  pub cover: Image,
}
