use chrono::NaiveDate;

use crate::entities::franchise::Franchise;
use crate::entities::image::Image;
use crate::enums::language::Language;
use crate::slug::Slug;

pub mod book_character;
pub mod book_involved;
pub mod book_statistic;
pub mod create_book;
pub mod create_partial_book;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct Book {
  pub id: u32,
  pub title: String,
  pub slug: Slug,
  pub description: Option<String>,
  pub published: Option<NaiveDate>,
  pub cover: Image,
  pub franchise: Option<Franchise>,
  pub language: Language,
}
