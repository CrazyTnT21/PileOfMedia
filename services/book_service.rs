use async_trait::async_trait;
use domain::entities::book::book::Book;
use domain::enums::language::Language;
use domain::items_total::ItemsTotal;
use domain::pagination::Pagination;
use crate::traits::service_error::ServiceError;

#[async_trait]
pub trait BookService: Send + Sync {
  async fn get(&self, language: Language, pagination: Pagination) -> Result<ItemsTotal<Book>, ServiceError>;
  async fn get_by_id(&self, id: u32, language: Language) -> Result<Option<Book>, ServiceError>;
  async fn get_by_title(&self, title: &str, language: Language, pagination: Pagination) -> Result<ItemsTotal<Book>, ServiceError>;
}
