use std::error::Error;
use std::fmt::{Display, Formatter};

use crate::join_comma::JoinComma;
use crate::traits::service_error::ServiceError;
use async_trait::async_trait;

#[async_trait]
pub trait MutBookGenreService: Send + Sync {
  async fn add(&self, book_id: u32, genres: &[u32]) -> Result<(), ServiceError<MutBookGenreServiceError>>;
  async fn remove(&self, book_id: u32, genres: &[u32]) -> Result<(), ServiceError<MutBookGenreServiceError>>;
}

#[derive(Debug)]
pub enum MutBookGenreServiceError {
  NonExistentBook(u32),
  AlreadyAssociated(Vec<u32>),
  NotAssociated(Vec<u32>),
  NonExistent(Vec<u32>),
  NoGenresProvided,
  OtherError(Box<dyn Error>),
}

impl Display for MutBookGenreServiceError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}",
      match self {
        MutBookGenreServiceError::NonExistentBook(x) => format!("Book with the id {x} does not exist"),
        MutBookGenreServiceError::AlreadyAssociated(x) =>
          format!("The following genres already have an association: [{}]", x.join_comma()),
        MutBookGenreServiceError::NotAssociated(x) =>
          format!("The following genres do not have an association: [{}]", x.join_comma()),
        MutBookGenreServiceError::NonExistent(x) => format!("The following genres do not exist: [{}]", x.join_comma()),
        MutBookGenreServiceError::NoGenresProvided => "No genres provided".to_string(),
        MutBookGenreServiceError::OtherError(x) => x.to_string(),
      }
    )
  }
}

impl Error for MutBookGenreServiceError {
  fn source(&self) -> Option<&(dyn Error + 'static)> {
    match self {
      MutBookGenreServiceError::OtherError(error) => Some(&**error),
      _ => None,
    }
  }
}
