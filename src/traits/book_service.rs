use std::error::Error;
use crate::application::page_count::PageCount;

use crate::domain::entities::book::book::Book;
use crate::domain::enums::language::Language;

pub trait BookService {
  fn get(&self, language: Language, fallback_language: Option<Language>, page_count: PageCount) -> Result<Vec<Book>, Box<dyn Error>>;
  fn get_by_id(&self, id: u32, language: Language, fallback_language: Option<Language>) -> Result<Option<Book>, Box<dyn Error>>;
  fn get_by_title(&self, title: &str, language: Language, fallback_language: Option<Language>, page_count: PageCount) -> Result<Vec<Book>, Box<dyn Error>>;
}
