use std::error::Error;

use chrono::NaiveDate;
use crate::application::pagination::Pagination;

use crate::domain::entities::book::book::Book;
use crate::domain::entities::franchise::franchise::Franchise;
use crate::domain::entities::image::image::{Image, ImageExtension};
use crate::domain::enums::language::Language;
use crate::traits::book_repository::BookRepository;

pub struct DefaultBookRepository;

impl BookRepository for DefaultBookRepository {
  fn get(&self, language: Language, fallback_language: Option<Language>, pagination: Pagination) -> Result<Vec<Book>, Box<dyn Error>> {
    Ok(vec![fake_book()])
  }

  fn get_by_id(&self, id: u32, language: Language, fallback_language: Option<Language>) -> Result<Option<Book>, Box<dyn Error>> {
    Ok(Some(fake_book()))
  }

  fn get_by_title(&self, title: &str, language: Language, fallback_language: Option<Language>, pagination: Pagination) -> Result<Vec<Book>, Box<dyn Error>> {
    Ok(vec![fake_book(), fake_book()])
  }
}

fn fake_book() -> Book {
  Book {
    cover: Image {
      extension: ImageExtension::JPG,
      height: 1002,
      id: 1,
      uri: String::from("https://Test"),
      width: 100,
    },
    id: 1,
    description: Some(String::from("Book description")),
    pages: Some(10),
    title: String::from("Book title"),
    words: Some(10),
    chapters: Some(1),
    published: NaiveDate::from_ymd_opt(2024, 2, 17),
    added: NaiveDate::MAX,
    members: 105,
    favorites: 2,
    rank: 6323,
    score: 6.25,
    popularity: 5545,
    franchise: Some(Franchise {
      id: 1,
      name: String::from("Book franchise"),
    }),
  }
}
