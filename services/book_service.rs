use std::fmt::{Display, Formatter};

use async_trait::async_trait;

use domain::entities::book::Book;
use domain::enums::language::Language;
use domain::items_total::ItemsTotal;
use domain::pagination::Pagination;

use crate::traits::service_error::ServiceError;

pub mod book_character_service;
pub mod book_genre_service;
pub mod book_involved_service;
pub mod book_theme_service;
pub mod mut_book_service;

#[async_trait]
pub trait BookService: Send + Sync {
  async fn get(
    &self,
    language: Language,
    pagination: Pagination,
  ) -> Result<ItemsTotal<Book>, ServiceError<BookServiceError>>;
  async fn get_by_id(&self, id: u32, language: Language) -> Result<Option<Book>, ServiceError<BookServiceError>>;
  async fn get_by_title(
    &self,
    title: &str,
    language: Language,
    pagination: Pagination,
  ) -> Result<ItemsTotal<Book>, ServiceError<BookServiceError>>;
}

pub enum BookServiceError {}

impl Display for BookServiceError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "")
  }
}
