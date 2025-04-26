use std::error::Error;
use std::fmt::{Display, Formatter};

use crate::join_comma::JoinComma;
use crate::traits::service_error::ServiceError;
use async_trait::async_trait;

#[async_trait]
pub trait MutBookThemeService: Send + Sync {
  async fn add(&self, book_id: u32, themes: &[u32]) -> Result<(), ServiceError<MutBookThemeServiceError>>;
  async fn remove(&self, book_id: u32, themes: &[u32]) -> Result<(), ServiceError<MutBookThemeServiceError>>;
}

#[derive(Debug)]
pub enum MutBookThemeServiceError {
  NonExistentBook(u32),
  AlreadyAssociated(Vec<u32>),
  NotAssociated(Vec<u32>),
  NonExistent(Vec<u32>),
  NoThemesProvided,
  OtherError(Box<dyn Error>),
}

impl Display for MutBookThemeServiceError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}",
      match self {
        MutBookThemeServiceError::NonExistentBook(x) => format!("Book with the id {x} does not exist"),
        MutBookThemeServiceError::AlreadyAssociated(x) =>
          format!("The following themes already have an association: [{}]", x.join_comma()),
        MutBookThemeServiceError::NotAssociated(x) =>
          format!("The following themes do not have an association: [{}]", x.join_comma()),
        MutBookThemeServiceError::NonExistent(x) => format!("The following themes do not exist: [{}]", x.join_comma()),
        MutBookThemeServiceError::NoThemesProvided => "No themes provided".to_string(),
        MutBookThemeServiceError::OtherError(x) => x.to_string(),
      }
    )
  }
}

impl Error for MutBookThemeServiceError {
  fn source(&self) -> Option<&(dyn Error + 'static)> {
    match self {
      MutBookThemeServiceError::OtherError(error) => Some(&**error),
      _ => None,
    }
  }
}
