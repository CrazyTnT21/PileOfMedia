use async_trait::async_trait;
use std::fmt::{Display, Formatter};

use domain::entities::account::{Account, Email, Password};
use domain::items_total::ItemsTotal;
use domain::pagination::Pagination;

use crate::traits::service_error::ServiceError;

pub mod mut_account_service;

#[async_trait]
pub trait AccountService: Send + Sync {
  async fn get(&self, pagination: Pagination) -> Result<ItemsTotal<Account>, ServiceError<AccountServiceError>>;
  async fn get_by_user_id(&self, id: u32) -> Result<Option<Account>, ServiceError<AccountServiceError>>;
  async fn get_by_email(&self, email: &Email) -> Result<Option<Account>, ServiceError<AccountServiceError>>;
  async fn login(&self, email: &Email, password: &Password) -> Result<Account, ServiceError<AccountServiceError>>;
}

pub enum AccountServiceError {
  UnknownEmail,
  InvalidEmail,
  WrongPassword,
  OtherError(Box<dyn Display>),
}

impl Display for AccountServiceError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}",
      match self {
        AccountServiceError::UnknownEmail => "Unknown email".to_string(),
        AccountServiceError::InvalidEmail => "Invalid email".to_string(),
        AccountServiceError::WrongPassword => "Wrong password".to_string(),
        AccountServiceError::OtherError(x) => x.to_string(),
      }
    )
  }
}
