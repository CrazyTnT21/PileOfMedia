use chrono::NaiveDate;

use crate::available_translations::AvailableTranslations;
use crate::entities::book::book_character::BookCharacter;
use crate::entities::book::book_statistic::BookStatistic;
use crate::entities::book::book_translation::BookTranslation;
use crate::entities::franchise::Franchise;
use crate::entities::genre::Genre;
use crate::entities::involved::Involved;
use crate::entities::theme::Theme;
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
  pub genres: Vec<Genre>,
  pub themes: Vec<Theme>,
  pub involved: Vec<Involved>,
  pub characters: Vec<BookCharacter>,
  pub translations: BookAvailableTranslations,
  pub statistic: BookStatistic,
}

type BookAvailableTranslations = AvailableTranslations<BookTranslation>;
