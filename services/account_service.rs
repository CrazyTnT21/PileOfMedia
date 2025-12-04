use async_trait::async_trait;
use std::error::Error;
use std::fmt::{Display, Formatter};

use domain::entities::account::{Account, Password};
use domain::items_total::ItemsTotal;
use domain::pagination::Pagination;

use crate::traits::service_error::ServiceError;

pub mod mut_account_service;

#[async_trait]
pub trait AccountService: Send + Sync {
  async fn get(&self, pagination: Pagination) -> Result<ItemsTotal<Account>, ServiceError<AccountServiceError>>;
  async fn get_by_user_id(&self, id: u32) -> Result<Option<Account>, ServiceError<AccountServiceError>>;
  async fn get_by_username(&self, name: &str) -> Result<Option<Account>, ServiceError<AccountServiceError>>;
  async fn login(&self, name: &str, password: &Password) -> Result<Account, ServiceError<AccountServiceError>>;
}

#[derive(Debug)]
pub enum AccountServiceError {
  UnknownUsernameOrInvalidPassword,
  OtherError(Box<dyn Error>),
}

impl Display for AccountServiceError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}",
      match self {
        AccountServiceError::UnknownUsernameOrInvalidPassword => "Unknown username or invalid password".to_string(),
        AccountServiceError::OtherError(x) => x.to_string(),
      }
    )
  }
}

impl Error for AccountServiceError {
  fn source(&self) -> Option<&(dyn Error + 'static)> {
    match self {
      AccountServiceError::UnknownUsernameOrInvalidPassword => None,
      AccountServiceError::OtherError(error) => Some(&**error),
    }
  }
}
