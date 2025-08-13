use std::error::Error;
use std::fmt::{Display, Formatter};

use async_trait::async_trait;

use crate::traits::service_error::ServiceError;
use domain::entities::genre::Genre;
use domain::enums::language::Language;

pub mod mut_book_genre_service;

#[async_trait]
pub trait BookGenreService: Send + Sync {
  async fn get_by_id(
    &self,
    book_id: u32,
    languages: &[Language],
  ) -> Result<Vec<Genre>, ServiceError<BookGenreServiceError>>;
}
#[derive(Debug)]
pub enum BookGenreServiceError {
  OtherError(Box<dyn Error>),
}

impl Display for BookGenreServiceError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}",
      match self {
        BookGenreServiceError::OtherError(value) => value.to_string(),
      }
    )
  }
}

impl Error for BookGenreServiceError {
  fn source(&self) -> Option<&(dyn Error + 'static)> {
    match self {
      BookGenreServiceError::OtherError(error) => Some(&**error),
    }
  }
}
