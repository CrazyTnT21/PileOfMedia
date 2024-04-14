use std::error::Error;
use async_trait::async_trait;
use crate::application::pagination::Pagination;

use crate::domain::entities::book::book::Book;
use crate::domain::enums::language::Language;

#[async_trait]
pub trait BookRepository: Send + Sync {
  async fn get(&self, language: Language, pagination: Pagination) -> Result<Vec<Book>, Box<dyn Error>>;
  async fn get_by_id(&self, id: u32, language: Language) -> Result<Option<Book>, Box<dyn Error>>;
  async fn get_by_title(&self, title: &str, language: Language, pagination: Pagination) -> Result<Vec<Book>, Box<dyn Error>>;
}
