use chrono::NaiveDate;
use tokio_postgres::Row;

use domain::entities::book::Book;
use domain::entities::franchise::Franchise;
use domain::entities::image::Image;
use domain::slug::Slug;
use from_row::FromRow;

use crate::schemas::db_book_translation::DbBookTranslation;

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
  pub fn to_entity(self, book_translation: DbBookTranslation, cover: Image, franchise: Option<Franchise>) -> Book {
    Book {
      id: self.id as u32,
      title: book_translation.title,
      slug: Slug::parse(self.slug).unwrap(),
      description: book_translation.description,
      published: self.published,
      cover,
      franchise,
      language: book_translation.language.into(),
    }
  }
}
