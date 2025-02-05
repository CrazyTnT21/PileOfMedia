use chrono::NaiveDate;

use crate::available_translations::AvailableTranslations;
use crate::entities::book::book_translation::BookTranslation;
use crate::entities::franchise::Franchise;
use crate::slug::Slug;

pub mod book_character;
pub mod book_involved;
pub mod book_statistic;
pub mod book_translation;
pub mod create_book;
pub mod create_partial_book;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct Book {
  pub id: u32,
  pub slug: Slug,
  pub published: Option<NaiveDate>,
  pub franchise: Option<Franchise>,
  pub translations: BookAvailableTranslations,
}

type BookAvailableTranslations = AvailableTranslations<BookTranslation>;
