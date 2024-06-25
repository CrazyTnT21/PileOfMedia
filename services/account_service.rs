use async_trait::async_trait;

use domain::entities::account::{Account, Email, Password};
use domain::items_total::ItemsTotal;
use domain::pagination::Pagination;

use crate::traits::service_error::ServiceError;

pub mod mut_account_service;

#[async_trait]
pub trait AccountService: Send + Sync {
  async fn get(&self, pagination: Pagination) -> Result<ItemsTotal<Account>, ServiceError>;
  async fn get_by_user_id(&self, id: u32) -> Result<Option<Account>, ServiceError>;
  async fn get_by_email(&self, email: &Email) -> Result<Option<Account>, ServiceError>;
  async fn login(&self, email: &Email, password: &Password) -> Result<Account, ServiceError>;
}
