use async_trait::async_trait;
use crate::application::pagination::Pagination;
use crate::domain::entities::book::book::Book;
use crate::domain::enums::language::Language;
use crate::services::traits::service_error::ServiceError;

#[async_trait]
pub trait BookService: Send + Sync {
  async fn get(&self, language: Language, pagination: Pagination) -> Result<Vec<Book>, ServiceError>;
  async fn get_by_id(&self, id: u32, language: Language) -> Result<Option<Book>, ServiceError>;
  async fn get_by_title(&self, title: &str, language: Language, pagination: Pagination) -> Result<Vec<Book>, ServiceError>;
}
