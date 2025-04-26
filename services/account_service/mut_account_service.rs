use async_trait::async_trait;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

use crate::traits::service_error::ServiceError;
use domain::entities::account::Account;
use domain::entities::account::create_account::CreateAccount;

#[async_trait]
pub trait MutAccountService: Send + Sync {
  async fn create(&self, account: CreateAccount) -> Result<Account, ServiceError<MutAccountServiceError>>;
}
#[derive(Debug)]
pub enum MutAccountServiceError {
  EmailAlreadyExists,
  InvalidEmail,
  InvalidPassword,
  OtherError(Box<dyn Error>),
}

impl Display for MutAccountServiceError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}",
      match self {
        MutAccountServiceError::EmailAlreadyExists => "Account with the given email already exists".to_string(),
        MutAccountServiceError::InvalidEmail => "Invalid email".to_string(),
        MutAccountServiceError::InvalidPassword => "Invalid password".to_string(),
        MutAccountServiceError::OtherError(x) => x.to_string(),
      }
    )
  }
}

impl Error for MutAccountServiceError {
  fn source(&self) -> Option<&(dyn Error + 'static)> {
    match self {
      MutAccountServiceError::OtherError(error) => Some(&**error),
      _ => None,
    }
  }
}
