// use crate::entities::book::book_edition::CreateBookEdition;
use crate::entities::image::Image;
use crate::entities::involved::InvolvedId;
use crate::enums::language::Language;
use crate::slug::Slug;
use chrono::NaiveDate;
use std::collections::HashMap;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreatePartialBook {
  pub slug: Slug,
  pub published: Option<NaiveDate>,
  pub franchise: Option<u32>,
  pub translations: HashMap<Language, CreatePartialBookTranslation>,
  pub genres: Vec<u32>,
  pub themes: Vec<u32>,
  pub characters: Vec<u32>,
  pub involved: Vec<InvolvedId>,
  // pub editions: Vec<CreateBookEdition>,
  // pub images: Vec<Image>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreatePartialBookTranslation {
  pub title: String,
  pub description: Option<String>,
  pub cover: Image,
}
