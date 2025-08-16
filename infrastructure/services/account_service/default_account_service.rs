use std::sync::Arc;

use argon2::{Argon2, PasswordHash, PasswordVerifier};
use async_trait::async_trait;
use domain::entities::account::{Account, Email, Password};
use domain::items_total::ItemsTotal;
use domain::pagination::Pagination;
use repositories::account_repository::AccountRepository;
use services::account_service::{AccountService, AccountServiceError};
use services::traits::service_error::ServiceError;

use crate::services::map_server_error;

pub struct DefaultAccountService<'a> {
  account_repository: Arc<dyn AccountRepository + 'a>,
}

impl<'a> DefaultAccountService<'a> {
  pub fn new(account_repository: Arc<dyn AccountRepository + 'a>) -> DefaultAccountService<'a> {
    DefaultAccountService { account_repository }
  }
}

#[async_trait]
impl AccountService for DefaultAccountService<'_> {
  async fn get(&self, pagination: Pagination) -> Result<ItemsTotal<Account>, ServiceError<AccountServiceError>> {
    Ok(self.account_repository.get(pagination).await?)
  }

  async fn get_by_user_id(&self, id: u32) -> Result<Option<Account>, ServiceError<AccountServiceError>> {
    Ok(self.account_repository.get_by_user_id(id).await?)
  }

  async fn get_by_email(&self, email: &Email) -> Result<Option<Account>, ServiceError<AccountServiceError>> {
    Ok(self.account_repository.get_by_email(email).await?)
  }

  async fn login(
    &self,
    email: &Email,
    given_password: &Password,
  ) -> Result<Account, ServiceError<AccountServiceError>> {
    let account = self
      .get_by_email(email)
      .await?
      .ok_or_else(unknown_email_or_invalid_password)?;

    let hash = password_hash(&account.password.0)?;

    let verified_password = Argon2::default().verify_password(given_password.0.as_bytes(), &hash);

    match verified_password {
      Ok(()) => Ok(account),
      Err(argon2::password_hash::Error::Password) => Err(unknown_email_or_invalid_password()),
      Err(e) => Err(ServiceError::ServerError(Box::new(e))),
    }
  }
}

fn password_hash(argon_password_hash: &str) -> Result<PasswordHash<'_>, ServiceError<AccountServiceError>> {
  PasswordHash::new(argon_password_hash).map_err(|y| map_server_error(Box::new(y)))
}

const fn unknown_email_or_invalid_password() -> ServiceError<AccountServiceError> {
  ServiceError::ClientError(AccountServiceError::UnknownEmailOrInvalidPassword)
}
