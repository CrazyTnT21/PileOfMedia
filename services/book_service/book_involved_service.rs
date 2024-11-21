use std::fmt::{Display, Formatter};

use async_trait::async_trait;

use domain::entities::book::book_involved::BookInvolved;
use domain::enums::language::Language;
use domain::items_total::ItemsTotal;
use domain::pagination::Pagination;

use crate::traits::service_error::ServiceError;

pub mod mut_book_involved_service;

#[async_trait]
pub trait BookInvolvedService: Send + Sync {
  async fn get(
    &self,
    book_id: u32,
    language: Language,
    pagination: Pagination,
  ) -> Result<ItemsTotal<BookInvolved>, ServiceError<BookInvolvedServiceError>>;
}

pub enum BookInvolvedServiceError {}

impl Display for BookInvolvedServiceError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "")
  }
}
