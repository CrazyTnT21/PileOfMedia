use chrono::NaiveDate;

use crate::entities::franchise::Franchise;
use crate::entities::image::Image;
use crate::enums::language::Language;

pub mod insert_book;
pub mod book_character;
pub mod book_involved;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct Book {
  pub id: u32,
  pub title: String,
  pub description: Option<String>,
  pub chapters: Option<u16>,
  pub pages: Option<u16>,
  pub words: Option<u32>,
  pub published: Option<NaiveDate>,
  pub cover: Image,
  pub score: f32,
  pub rank: u32,
  pub popularity: u32,
  pub favorites: u32,
  pub members: u32,
  pub added: NaiveDate,
  pub franchise: Option<Franchise>,
  pub language: Language
}

