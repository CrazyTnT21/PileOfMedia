use std::fmt::{Display, Formatter};

use async_trait::async_trait;

use crate::join_comma::JoinComma;
use crate::traits::service_error::ServiceError;
use domain::entities::book::book_statistic::BookStatistic;
use domain::entities::book::Book;
use domain::enums::language::Language;
use domain::items_total::ItemsTotal;
use domain::pagination::Pagination;
use domain::slug::Slug;

pub mod book_character_service;
pub mod book_genre_service;
pub mod book_involved_service;
pub mod book_theme_service;
pub mod mut_book_service;

#[async_trait]
pub trait BookService: Send + Sync {
  async fn get(
    &self,
    languages: &[Language],
    pagination: Pagination,
  ) -> Result<ItemsTotal<Book>, ServiceError<BookServiceError>>;
  async fn get_by_id(&self, id: u32, languages: &[Language]) -> Result<Option<Book>, ServiceError<BookServiceError>>;
  async fn get_by_title(
    &self,
    title: &str,
    languages: &[Language],
    pagination: Pagination,
  ) -> Result<ItemsTotal<Book>, ServiceError<BookServiceError>>;

  async fn get_by_slug(
    &self,
    slug: &Slug,
    languages: &[Language],
  ) -> Result<Option<Book>, ServiceError<BookServiceError>>;
  async fn get_statistics(&self, book_ids: &[u32]) -> Result<Vec<BookStatistic>, ServiceError<BookServiceError>>;
}

pub enum BookServiceError {
  NonExistentBooks(Vec<u32>),
}

impl Display for BookServiceError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}",
      match self {
        BookServiceError::NonExistentBooks(x) =>
          format!("Books with the following ids do not exist: [{}]", x.join_comma()),
      }
    )
  }
}
