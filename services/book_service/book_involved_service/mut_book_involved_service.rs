use std::error::Error;
use std::fmt::{Display, Formatter};

use async_trait::async_trait;

use crate::join_comma::JoinComma;
use crate::traits::service_error::ServiceError;
use domain::entities::involved::InvolvedId;

#[async_trait]
pub trait MutBookInvolvedService: Send + Sync {
  async fn add(&self, book_id: u32, involved: &[InvolvedId]) -> Result<(), ServiceError<MutBookInvolvedServiceError>>;
  async fn remove(
    &self,
    book_id: u32,
    involved: &[InvolvedId],
  ) -> Result<(), ServiceError<MutBookInvolvedServiceError>>;
}

#[derive(Debug)]
pub enum MutBookInvolvedServiceError {
  NonExistentBook(u32),
  AlreadyAssociated(Vec<InvolvedId>),
  NonExistentAssociation(Vec<InvolvedId>),
  NonExistentPeople(Vec<u32>),
  NonExistentRoles(Vec<u32>),
  NoInvolvedProvided,
  OtherError(Box<dyn Error>),
}

impl Display for MutBookInvolvedServiceError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}",
      match self {
        MutBookInvolvedServiceError::NonExistentBook(x) => format!("Book with the id {x} does not exist"),
        MutBookInvolvedServiceError::AlreadyAssociated(x) => format!(
          "The following people with roles already have an association: [{}]",
          x.join_comma()
        ),
        MutBookInvolvedServiceError::NonExistentAssociation(x) => format!(
          "The following people with roles do not have an association: [{}]",
          x.join_comma()
        ),
        MutBookInvolvedServiceError::NonExistentPeople(x) =>
          format!("The following people do not exist: [{}]", x.join_comma()),
        MutBookInvolvedServiceError::NonExistentRoles(x) =>
          format!("The following roles do not exist: [{}]", x.join_comma()),
        MutBookInvolvedServiceError::NoInvolvedProvided => "No involved provided".to_string(),
        MutBookInvolvedServiceError::OtherError(x) => x.to_string(),
      }
    )
  }
}

impl Error for MutBookInvolvedServiceError {
  fn source(&self) -> Option<&(dyn Error + 'static)> {
    match self {
      MutBookInvolvedServiceError::OtherError(error) => Some(&**error),
      _ => None,
    }
  }
}
