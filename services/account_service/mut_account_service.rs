use async_trait::async_trait;

use domain::entities::account::create_account::CreateAccount;
use domain::entities::account::Account;

use crate::traits::service_error::ServiceError;

#[async_trait]
pub trait MutAccountService: Send + Sync {
  async fn create(&self, account: CreateAccount) -> Result<Account, ServiceError>;
}
