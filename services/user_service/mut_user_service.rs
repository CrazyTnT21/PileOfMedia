use std::fmt::{Display, Formatter};

use async_trait::async_trait;

use domain::entities::user::User;
use domain::entities::user::create_user::CreateUser;

use crate::traits::service_error::ServiceError;

#[async_trait]
pub trait MutUserService: Send + Sync {
  async fn create(&self, user: CreateUser) -> Result<User, ServiceError<MutUserServiceError>>;
}

pub enum MutUserServiceError {
  OtherError(Box<dyn Display>),
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
