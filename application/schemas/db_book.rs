use chrono::NaiveDate;
use domain::available_translations::AvailableTranslations;
use domain::entities::book::book_translation::BookTranslation;
use domain::entities::book::Book;
use domain::entities::franchise::Franchise;
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
  pub fn to_entity(self, translations: AvailableTranslations<BookTranslation>, franchise: Option<Franchise>) -> Book {
    Book {
      id: self.id as u32,
      slug: Slug::parse(self.slug).unwrap(),
      published: self.published,
      franchise,
      translations,
    }
  }
}
