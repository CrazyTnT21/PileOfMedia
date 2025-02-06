use chrono::NaiveDate;
use domain::available_translations::AvailableTranslations;
use domain::entities::book::book_character::BookCharacter;
use domain::entities::book::book_involved::BookInvolved;
use domain::entities::book::book_statistic::BookStatistic;
use domain::entities::book::book_translation::BookTranslation;
use domain::entities::book::Book;
use domain::entities::franchise::Franchise;
use domain::entities::genre::Genre;
use domain::entities::theme::Theme;
use domain::slug::Slug;
use from_row::FromRow;
use tokio_postgres::Row;

#[derive(FromRow, Debug)]
#[rename = "book"]
pub struct DbBook {
  pub id: i32,
  pub published: Option<NaiveDate>,
  pub slug: String,
  #[rename = "fkfranchise"]
  pub fk_franchise: Option<i32>,
}

impl DbBook {
  /// # Panics
  ///
  /// Will panic if the book slug is not valid. This could only happen if the value was not validated when inserted.
  pub fn to_entity(
    self,
    translations: AvailableTranslations<BookTranslation>,
    franchise: Option<Franchise>,
    genres: Vec<Genre>,
    themes: Vec<Theme>,
    involved: Vec<BookInvolved>,
    characters: Vec<BookCharacter>,
    statistic: BookStatistic,
  ) -> Book {
    Book {
      id: self.id as u32,
      slug: Slug::parse(self.slug).unwrap(),
      published: self.published,
      franchise,
      translations,
      genres,
      themes,
      involved,
      characters,
      statistic,
    }
  }
}
