use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Display, Formatter};

use async_trait::async_trait;
use domain::entities::user::user_book::UserBook;
use domain::enums::language::Language;

use crate::traits::service_error::ServiceError;

pub mod mut_user_book_service;

#[async_trait]
pub trait UserBookService: Send + Sync {
  async fn get_by_book_id(
    &self,
    user_id: u32,
    book_id: u32,
    languages: &[Language],
  ) -> Result<Option<UserBook>, ServiceError<UserBookServiceError>>;
  async fn get_by_book_ids(
    &self,
    user_id: u32,
    book_ids: &[u32],
    languages: &[Language],
  ) -> Result<Vec<UserBook>, ServiceError<UserBookServiceError>>;
  async fn get_by_user_id(
    &self,
    user_id: u32,
    languages: &[Language],
  ) -> Result<Vec<UserBook>, ServiceError<UserBookServiceError>>;
  async fn get_by_user_ids(
    &self,
    user_ids: &[u32],
    languages: &[Language],
  ) -> Result<HashMap<u32, Vec<UserBook>>, ServiceError<UserBookServiceError>>;
}

#[derive(Debug)]
pub enum UserBookServiceError {
  OtherError(Box<dyn Error>),
}

impl Display for UserBookServiceError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}",
      match self {
        UserBookServiceError::OtherError(value) => value.to_string(),
      }
    )
  }
}

impl Error for UserBookServiceError {
  fn source(&self) -> Option<&(dyn Error + 'static)> {
    match self {
      UserBookServiceError::OtherError(error) => Some(&**error),
    }
  }
}
