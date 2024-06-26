use std::fmt::{Display, Formatter};

use async_trait::async_trait;

use crate::join_comma::JoinComma;
use crate::traits::service_error::ServiceError;

#[async_trait]
pub trait MutBookCharacterService: Send + Sync {
  async fn add(&self, book_id: u32, characters: &[u32]) -> Result<(), ServiceError<MutBookCharacterServiceError>>;
  async fn remove(&self, book_id: u32, characters: &[u32]) -> Result<(), ServiceError<MutBookCharacterServiceError>>;
}

pub enum MutBookCharacterServiceError {
  NonExistentBook(u32),
  AlreadyAssociated(Vec<u32>),
  NotAssociated(Vec<u32>),
  NonExistent(Vec<u32>),
  NoCharactersProvided,
  OtherError(Box<dyn Display>),
}

impl Display for MutBookCharacterServiceError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", match self {
      MutBookCharacterServiceError::NonExistentBook(x) => format!("Book with the id {x} does not exist"),
      MutBookCharacterServiceError::AlreadyAssociated(x) => format!("The following characters already have an association: [{}]", x.join_comma()),
      MutBookCharacterServiceError::NotAssociated(x) => format!("The following characters do not have an association: [{}]", x.join_comma()),
      MutBookCharacterServiceError::NonExistent(x) => format!("The following characters do not exist: [{}]", x.join_comma()),
      MutBookCharacterServiceError::NoCharactersProvided => "No characters provided".to_string(),
      MutBookCharacterServiceError::OtherError(x) => x.to_string()
    })
  }
}
