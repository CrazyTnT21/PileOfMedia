use std::error::Error;
use std::fmt::{Display, Formatter};

use async_trait::async_trait;

use domain::entities::user::User;
use domain::entities::user::create_user::CreateUser;

use crate::traits::service_error::ServiceError;

#[async_trait]
pub trait MutUserService: Send + Sync {
  async fn create(&self, user: CreateUser) -> Result<User, ServiceError<MutUserServiceError>>;
}

#[derive(Debug)]
pub enum MutUserServiceError {
  OtherError(Box<dyn Error>),
}

impl Display for MutUserServiceError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}",
      match self {
        MutUserServiceError::OtherError(x) => x.to_string(),
      }
    )
  }
}

impl Error for MutUserServiceError {
  fn source(&self) -> Option<&(dyn Error + 'static)> {
    match self {
      MutUserServiceError::OtherError(error) => Some(&**error),
    }
  }
}
