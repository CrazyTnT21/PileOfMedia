use async_trait::async_trait;

use domain::entities::account::partial_create_account::PartialCreateAccount;
use domain::entities::account::Account;

use crate::traits::service_error::ServiceError;

#[async_trait]
pub trait MutAccountService: Send + Sync {
  async fn create(&self, account: PartialCreateAccount) -> Result<Account, ServiceError>;
}
