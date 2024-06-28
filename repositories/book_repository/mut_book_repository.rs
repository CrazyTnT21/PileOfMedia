use std::error::Error;
use async_trait::async_trait;
use domain::entities::book::Book;
use domain::entities::book::create_partial_book::CreatePartialBook;

#[async_trait]
pub trait MutBookRepository: Send + Sync {
  async fn create(&self, item: CreatePartialBook) -> Result<Book, Box<dyn Error>>;
  async fn delete(&self, ids: &[u32]) -> Result<(), Box<dyn Error>>;
}
