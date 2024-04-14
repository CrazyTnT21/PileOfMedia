use std::error::Error;
use std::sync::Arc;

use crate::application::pagination::Pagination;
use crate::domain::entities::book::book::Book;
use crate::domain::enums::language::Language;
use crate::repositories::book_repository::BookRepository;
use crate::services::book_service::BookService;
use crate::services::traits::service_error::ServiceError;

pub struct DefaultBookService {
  book_repository: Arc<dyn BookRepository>,
}

impl DefaultBookService {
  pub fn new(book_repository: Arc<dyn BookRepository>) -> DefaultBookService {
    DefaultBookService { book_repository }
  }
}

impl BookService for DefaultBookService {
  fn get(&self, language: Language, pagination: Pagination) -> Result<Vec<Book>, ServiceError> {
    self.book_repository.get(language, pagination).map_err(map_server_error)
  }

  fn get_by_id(&self, id: u32, language: Language) -> Result<Option<Book>, ServiceError> {
    self.book_repository.get_by_id(id, language).map_err(map_server_error)
  }

  fn get_by_title(&self, title: &str, language: Language, pagination: Pagination) -> Result<Vec<Book>, ServiceError> {
    self.book_repository.get_by_title(title, language, pagination).map_err(map_server_error)
  }
}

fn map_server_error(error: Box<dyn Error>) -> ServiceError {
  ServiceError::ServerError(error)
}
