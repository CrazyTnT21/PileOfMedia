use std::error::Error;
use std::fmt::{Display, Formatter};

use crate::traits::service_error::ServiceError;
use async_trait::async_trait;
use domain::entities::book::book_involved::BookInvolved;
use domain::enums::language::Language;

pub mod mut_book_involved_service;

#[async_trait]
pub trait BookInvolvedService: Send + Sync {
  async fn get_by_id(
    &self,
    book_id: u32,
    languages: &[Language],
  ) -> Result<Vec<BookInvolved>, ServiceError<BookInvolvedServiceError>>;
}
#[derive(Debug)]
pub enum BookInvolvedServiceError {
  OtherError(Box<dyn Error>),
}

impl Display for BookInvolvedServiceError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}",
      match self {
        BookInvolvedServiceError::OtherError(value) => value.to_string(),
      }
    )
  }
}

impl Error for BookInvolvedServiceError {
  fn source(&self) -> Option<&(dyn Error + 'static)> {
    match self {
      BookInvolvedServiceError::OtherError(error) => Some(&**error),
    }
  }
}
