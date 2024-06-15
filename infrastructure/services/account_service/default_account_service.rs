use std::sync::Arc;
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use async_trait::async_trait;

use domain::entities::account::{Account, Email, Password};
use domain::items_total::ItemsTotal;
use domain::pagination::Pagination;
use repositories::account_repository::AccountRepository;
use services::account_service::AccountService;
use services::traits::service_error::{ClientError, ServiceError};

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
impl<'a> AccountService for DefaultAccountService<'a> {
  async fn get(&self, pagination: Pagination) -> Result<ItemsTotal<Account>, ServiceError> {
    self.account_repository.get(pagination).await.map_err(map_server_error)
  }

  async fn get_by_user_id(&self, id: u32) -> Result<Option<Account>, ServiceError> {
    self.account_repository.get_by_user_id(id).await.map_err(map_server_error)
  }

  async fn get_by_user_ids(&self, ids: &[u32]) -> Result<Vec<Account>, ServiceError> {
    self.account_repository.get_by_user_ids(ids).await.map_err(map_server_error)
  }

  async fn get_by_email(&self, email: &Email) -> Result<Option<Account>, ServiceError> {
    self.account_repository.get_by_email(email).await.map_err(map_server_error)
  }

  async fn login(&self, email: &Email, password: &Password) -> Result<Account, ServiceError> {
    let account = self.get_by_email(email)
      .await?
      .ok_or(unknown_email())?;

    let hash = password_hash(&account.password.0)?;
    if Argon2::default().verify_password(password.0.as_bytes(), &hash).is_ok() {
      return Ok(account);
    }
    Err(wrong_password())
  }
}

fn password_hash(argon_password: &str) -> Result<PasswordHash, ServiceError> {
  PasswordHash::new(argon_password).map_err(|y| map_server_error(Box::new(y)))
}

fn unknown_email() -> ServiceError {
  ServiceError::ClientError(ClientError {
    title: "Unknown email".to_string(),
    description: None,
  })
}

fn wrong_password() -> ServiceError {
  ServiceError::ClientError(ClientError
  {
    title: "Wrong password".to_string(),
    description: None,
  })
}
