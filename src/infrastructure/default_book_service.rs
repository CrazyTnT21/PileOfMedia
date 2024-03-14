use std::error::Error;
use crate::application::page_count::PageCount;

use crate::domain::entities::book::book::Book;
use crate::domain::enums::language::Language;
use crate::traits::book_repository::BookRepository;
use crate::traits::book_service::BookService;

pub struct DefaultBookService {
  book_repository: Box<dyn BookRepository>,
}

impl DefaultBookService {
  pub fn new(book_repository: Box<dyn BookRepository>) -> DefaultBookService {
    DefaultBookService { book_repository }
  }
}

impl BookService for DefaultBookService {
  fn get(&self, language: Language, fallback_language: Option<Language>, page_count: PageCount) -> Result<Vec<Book>, Box<dyn Error>> {
    self.book_repository.get(language, fallback_language, page_count)
  }

  fn get_by_id(&self, id: u32, language: Language, fallback_language: Option<Language>) -> Result<Option<Book>, Box<dyn Error>> {
    self.book_repository.get_by_id(id, language, fallback_language)
  }

  fn get_by_title(&self, title: &str, language: Language, fallback_language: Option<Language>, page_count: PageCount) -> Result<Vec<Book>, Box<dyn Error>> {
    self.book_repository.get_by_title(title, language, fallback_language, page_count)
  }
}
