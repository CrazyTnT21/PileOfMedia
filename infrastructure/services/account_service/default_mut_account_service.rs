use std::sync::Arc;

use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHasher};
use async_trait::async_trait;

use domain::entities::account::create_account::CreateAccount;
use domain::entities::account::create_partial_account::CreatePartialAccount;
use domain::entities::account::{Account, Password};
use domain::entities::user::create_user::CreateUser;
use repositories::account_repository::mut_account_repository::MutAccountRepository;
use services::account_service::mut_account_service::MutAccountServiceError::OtherError;
use services::account_service::mut_account_service::{MutAccountService, MutAccountServiceError};
use services::account_service::AccountService;
use services::traits::service_error::ServiceError;
use services::traits::service_error::ServiceError::{ClientError, ServerError};
use services::user_service::mut_user_service::MutUserService;

use crate::services::map_server_error;

pub struct DefaultMutAccountService<'a> {
  mut_account_repository: Arc<dyn MutAccountRepository + 'a>,
  account_service: Arc<dyn AccountService + 'a>,
  mut_user_service: Arc<dyn MutUserService + 'a>,
}

impl<'a> DefaultMutAccountService<'a> {
  pub fn new(
    mut_account_repository: Arc<dyn MutAccountRepository + 'a>,
    account_repository: Arc<dyn AccountService + 'a>,
    mut_user_service: Arc<dyn MutUserService + 'a>,
  ) -> DefaultMutAccountService<'a> {
    DefaultMutAccountService {
      mut_account_repository,
      account_service: account_repository,
      mut_user_service,
    }
  }
}

#[async_trait]
impl<'a> MutAccountService for DefaultMutAccountService<'a> {
  async fn create(&self, account: CreateAccount) -> Result<Account, ServiceError<MutAccountServiceError>> {
    self.validate_create(&account).await?;
    let data = account.account;
    let user = CreateUser {
      user: data.user,
      profile_picture: account.profile_picture,
    };
    let user = self.mut_user_service.create(user).await.map_err(|x| match x {
      ClientError(x) => ClientError(OtherError(Box::new(x))),
      ServerError(x) => ServerError(x),
    })?;
    let account = CreatePartialAccount {
      user,
      email: data.email,
      password: hash_password(&data.password.0)?,
    };
    Ok(self.mut_account_repository.create(account).await?)
  }
}

impl<'a> DefaultMutAccountService<'a> {
  async fn validate_create(&self, account: &CreateAccount) -> Result<(), ServiceError<MutAccountServiceError>> {
    let data = &account.account;
    if data.email.0.is_empty() {
      return Err(ClientError(MutAccountServiceError::InvalidEmail));
    };
    if data.password.0.is_empty() {
      return Err(ClientError(MutAccountServiceError::InvalidPassword));
    }
    let exists_email = self
      .account_service
      .get_by_email(&data.email)
      .await
      .map_err(|x| match x {
        ClientError(x) => ClientError(OtherError(Box::new(x))),
        ServerError(x) => ServerError(x),
      })?;
    if exists_email.is_some() {
      return Err(ClientError(MutAccountServiceError::EmailAlreadyExists));
    };
    Ok(())
  }
}

fn hash_password(password: &str) -> Result<Password, ServiceError<MutAccountServiceError>> {
  let salt = SaltString::generate(&mut OsRng);
  let argon2 = Argon2::default();
  let password_hash = argon2
    .hash_password(password.as_bytes(), &salt)
    .map_err(|y| map_server_error(Box::new(y)))?
    .to_string();
  Ok(Password(password_hash))
}
