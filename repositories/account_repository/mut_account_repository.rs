use std::error::Error;

use async_trait::async_trait;

use domain::entities::account::create_account::CreateAccount;
use domain::entities::account::Account;

#[async_trait]
pub trait MutAccountRepository: Send + Sync {
  async fn create(&self, account: CreateAccount) -> Result<Account, Box<dyn Error>>;
}
