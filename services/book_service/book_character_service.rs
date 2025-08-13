use std::error::Error;
use std::fmt::{Display, Formatter};

use crate::traits::service_error::ServiceError;
use async_trait::async_trait;
use domain::entities::book::book_character::BookCharacter;
use domain::enums::language::Language;

pub mod mut_book_character_service;

#[async_trait]
pub trait BookCharacterService: Send + Sync {
  async fn get_by_id(
    &self,
    book_id: u32,
    languages: &[Language],
  ) -> Result<Vec<BookCharacter>, ServiceError<BookCharacterServiceError>>;
}
#[derive(Debug)]
pub enum BookCharacterServiceError {
  OtherError(Box<dyn Error>),
}

impl Display for BookCharacterServiceError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}",
      match self {
        BookCharacterServiceError::OtherError(value) => value.to_string(),
      }
    )
  }
}

impl Error for BookCharacterServiceError {
  fn source(&self) -> Option<&(dyn Error + 'static)> {
    match self {
      BookCharacterServiceError::OtherError(error) => Some(&**error),
    }
  }
}
