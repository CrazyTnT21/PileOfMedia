use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Display, Formatter};

use crate::join_comma::JoinComma;
use crate::traits::service_error::ServiceError;
use async_trait::async_trait;
use domain::entities::user::create_user_book::CreateUserBook;
use domain::entities::user::user_book::UserBook;
use domain::enums::language::Language;

#[async_trait]
pub trait MutUserBookService: Send + Sync {
  async fn add(
    &self,
    user_id: u32,
    book: CreateUserBook,
    languages: &[Language],
  ) -> Result<UserBook, ServiceError<MutUserBookServiceError>>;
  async fn remove(&self, user_id: u32, book_id: &[u32]) -> Result<(), ServiceError<MutUserBookServiceError>>;
}

#[derive(Debug)]
pub enum MutUserBookServiceError {
  NonExistentUser(u32),
  AlreadyAssociated(HashMap<u32, Vec<u32>>),
  NotAssociated(Vec<u32>),
  NonExistent(Vec<u32>),
  OtherError(Box<dyn Error>),
}

impl Display for MutUserBookServiceError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}",
      match self {
        MutUserBookServiceError::NonExistentUser(x) => format!("User with the id {x} does not exist"),
        MutUserBookServiceError::AlreadyAssociated(x) => format!(
          "The following books already have an association: [{}]",
          x.values().flatten().collect::<Vec<&u32>>().join_comma()
        ),
        MutUserBookServiceError::NotAssociated(x) =>
          format!("The following books do not have an association: [{}]", x.join_comma()),
        MutUserBookServiceError::NonExistent(x) => format!("The following books do not exist: [{}]", x.join_comma()),
        MutUserBookServiceError::OtherError(x) => x.to_string(),
      }
    )
  }
}

impl Error for MutUserBookServiceError {
  fn source(&self) -> Option<&(dyn Error + 'static)> {
    match self {
      MutUserBookServiceError::OtherError(error) => Some(&**error),
      _ => None,
    }
  }
}
