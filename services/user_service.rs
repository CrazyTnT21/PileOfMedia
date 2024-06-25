use async_trait::async_trait;

use domain::entities::user::User;
use domain::items_total::ItemsTotal;
use domain::pagination::Pagination;

use crate::traits::service_error::ServiceError;

pub mod mut_user_service;

#[async_trait]
pub trait UserService: Send + Sync {
  async fn get(&self, pagination: Pagination) -> Result<ItemsTotal<User>, ServiceError>;
  async fn get_by_id(&self, id: u32) -> Result<Option<User>, ServiceError>;
  async fn get_by_name(&self, name: &str, pagination: Pagination) -> Result<ItemsTotal<User>, ServiceError>;
}
