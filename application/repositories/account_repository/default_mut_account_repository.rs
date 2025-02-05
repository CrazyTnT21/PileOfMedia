use std::error::Error;
use std::sync::Arc;

use async_trait::async_trait;
use tokio_postgres::Transaction;

use crate::insert::Insert;
use crate::schemas::db_account::DbAccount;
use domain::entities::account::create_partial_account::CreatePartialAccount;
use domain::entities::account::Account;
use repositories::account_repository::mut_account_repository::MutAccountRepository;
use repositories::account_repository::AccountRepository;

pub struct DefaultMutAccountRepository<'a> {
  transaction: &'a Transaction<'a>,
  account_repository: Arc<dyn AccountRepository + 'a>,
}

impl<'a> DefaultMutAccountRepository<'a> {
  pub fn new(
    transaction: &'a Transaction<'a>,
    account_repository: Arc<dyn AccountRepository + 'a>,
  ) -> DefaultMutAccountRepository<'a> {
    DefaultMutAccountRepository {
      transaction,
      account_repository,
    }
  }
}

#[async_trait]
impl MutAccountRepository for DefaultMutAccountRepository<'_> {
  async fn create(&self, account: CreatePartialAccount) -> Result<Account, Box<dyn Error>> {
    let user_id = account.user.id as i32;
    let id: i32 = Insert::new::<DbAccount>(["fkuser", "email", "password"])
      .values([&user_id, &account.email.0, &account.password.0])
      .returning_transaction("fkuser", self.transaction)
      .await?;

    Ok(
      self
        .account_repository
        .get_by_user_id(id as u32)
        .await?
        .expect("Account was just created, it should exist"),
    )
  }
}
