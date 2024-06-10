use std::error::Error;

use async_trait::async_trait;

use domain::entities::account::{Account, Email};
use domain::items_total::ItemsTotal;
use domain::pagination::Pagination;

pub mod mut_account_repository;

#[async_trait]
pub trait AccountRepository: Send + Sync {
  async fn get(&self, pagination: Pagination) -> Result<ItemsTotal<Account>, Box<dyn Error>>;
  async fn get_by_user_id(&self, id: u32) -> Result<Option<Account>, Box<dyn Error>>;
  async fn get_by_user_ids(&self, ids: &[i32]) -> Result<Vec<Account>, Box<dyn Error>>;
  async fn get_by_email(&self, email: &Email) -> Result<Option<Account>, Box<dyn Error>>;
}
