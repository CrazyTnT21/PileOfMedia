use std::error::Error;
use std::fmt::{Display, Formatter};

use async_trait::async_trait;

use domain::entities::user::User;
use domain::items_total::ItemsTotal;
use domain::pagination::Pagination;

use crate::traits::service_error::ServiceError;

pub mod mut_user_service;
pub mod user_book_service;

#[async_trait]
pub trait UserService: Send + Sync {
  async fn get(&self, pagination: Pagination) -> Result<ItemsTotal<User>, ServiceError<UserServiceError>>;
  async fn get_by_id(&self, id: u32) -> Result<Option<User>, ServiceError<UserServiceError>>;
  async fn get_by_name(
    &self,
    name: &str,
    pagination: Pagination,
  ) -> Result<ItemsTotal<User>, ServiceError<UserServiceError>>;
  async fn get_by_username(&self, name: &str) -> Result<Option<User>, ServiceError<UserServiceError>>;
}

#[derive(Debug)]
pub enum UserServiceError {
  OtherError(Box<dyn Error>),
}

impl Display for UserServiceError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}",
      match self {
        UserServiceError::OtherError(value) => value.to_string(),
      }
    )
  }
}

impl Error for UserServiceError {
  fn source(&self) -> Option<&(dyn Error + 'static)> {
    match self {
      UserServiceError::OtherError(error) => Some(&**error),
    }
  }
}
