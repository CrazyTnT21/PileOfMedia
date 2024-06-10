use std::sync::Arc;
use argon2::{Argon2, PasswordHasher};
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use async_trait::async_trait;

use domain::entities::account::create_account::CreateAccount;
use domain::entities::account::partial_create_account::PartialCreateAccount;
use domain::entities::account::{Account, Password};
use repositories::account_repository::mut_account_repository::MutAccountRepository;
use services::account_service::AccountService;
use services::user_service::mut_user_service::MutUserService;
use services::traits::service_error::{ClientError, ServiceError};
use services::account_service::mut_account_service::MutAccountService;

use crate::services::map_server_error;

pub struct DefaultMutAccountService<'a> {
  mut_account_repository: Arc<dyn MutAccountRepository + 'a>,
  account_service: Arc<dyn AccountService + 'a>,
  mut_user_service: Arc<dyn MutUserService + 'a>,
}

impl<'a> DefaultMutAccountService<'a> {
  pub fn new(mut_account_repository: Arc<dyn MutAccountRepository + 'a>,
             account_repository: Arc<dyn AccountService + 'a>,
             mut_user_service: Arc<dyn MutUserService + 'a>, ) -> DefaultMutAccountService<'a> {
    DefaultMutAccountService { mut_account_repository, account_service: account_repository, mut_user_service }
  }
}

#[async_trait]
impl<'a> MutAccountService for DefaultMutAccountService<'a> {
  async fn create(&self, account: PartialCreateAccount) -> Result<Account, ServiceError> {
    self.validate_create(&account).await?;

    let user = self.mut_user_service.create(account.user).await?;
    let account = CreateAccount {
      user,
      email: account.email,
      password: hash_password(&account.password.0)?,
    };
    self.mut_account_repository.create(account).await.map_err(map_server_error)
  }
}

impl<'a> DefaultMutAccountService<'a> {
  async fn validate_create(&self, account: &PartialCreateAccount) -> Result<(), ServiceError> {
    if account.email.0.is_empty() {
      return Err(ServiceError::ClientError(ClientError {
        title: "No email was provided".to_string(),
        description: None,
      }));
    };
    if account.password.0.is_empty() {
      return Err(ServiceError::ClientError(ClientError {
        title: "No password was provided".to_string(),
        description: None,
      }));
    }
    let exists_email = self.account_service.get_by_email(&account.email).await?;
    if let Some(_) = exists_email {
      return Err(ServiceError::ClientError(ClientError {
        title: format!("Account with the email {} already exists", account.email.0),
        description: None,
      }));
    };
    Ok(())
  }
}

fn hash_password(password: &str) -> Result<Password, ServiceError> {
  let salt = SaltString::generate(&mut OsRng);
  let argon2 = Argon2::default();
  let password_hash = argon2.hash_password(password.as_bytes(), &salt).map_err(|y| map_server_error(Box::new(y)))?.to_string();
  Ok(Password(password_hash))
}
