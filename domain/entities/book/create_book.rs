use std::collections::HashMap;

use chrono::NaiveDate;

use crate::entities::book::book_involved::InvolvedId;
use crate::entities::image::create_image::CreateImage;
use crate::enums::language::Language;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreateBook {
  pub chapters: Option<u16>,
  pub pages: Option<u16>,
  pub words: Option<u32>,
  pub published: Option<NaiveDate>,
  pub franchise: Option<u32>,
  pub translations: HashMap<Language, CreateBookTranslation>,
  pub genres: Vec<u32>,
  pub themes: Vec<u32>,
  pub characters: Vec<u32>,
  pub involved: Vec<InvolvedId>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreateBookTranslation {
  pub title: String,
  pub description: Option<String>,
  pub cover: CreateCover,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum CreateCover {
  Image(CreateImage),
  ReuseFromLanguage(Language),
}
