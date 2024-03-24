use std::error::Error;
use crate::application::pagination::Pagination;

use crate::domain::entities::book::book::Book;
use crate::domain::enums::language::Language;

pub trait BookService: Send + Sync {
  fn get(&self, language: Language, fallback_language: Option<Language>, pagination: Pagination) -> Result<Vec<Book>, Box<dyn Error>>;
  fn get_by_id(&self, id: u32, language: Language, fallback_language: Option<Language>) -> Result<Option<Book>, Box<dyn Error>>;
  fn get_by_title(&self, title: &str, language: Language, fallback_language: Option<Language>, pagination: Pagination) -> Result<Vec<Book>, Box<dyn Error>>;
}
