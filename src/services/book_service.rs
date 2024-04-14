use crate::application::pagination::Pagination;
use crate::domain::entities::book::book::Book;
use crate::domain::enums::language::Language;
use crate::services::traits::service_error::ServiceError;

pub trait BookService: Send + Sync {
  fn get(&self, language: Language, pagination: Pagination) -> Result<Vec<Book>, ServiceError>;
  fn get_by_id(&self, id: u32, language: Language) -> Result<Option<Book>, ServiceError>;
  fn get_by_title(&self, title: &str, language: Language, pagination: Pagination) -> Result<Vec<Book>, ServiceError>;
}
