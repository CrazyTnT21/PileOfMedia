use std::error::Error;

use async_trait::async_trait;

use domain::entities::user::User;
use domain::items_total::ItemsTotal;
use domain::pagination::Pagination;

pub mod mut_user_repository;

#[async_trait]
pub trait UserRepository: Send + Sync {
  async fn get(&self, pagination: Pagination) -> Result<ItemsTotal<User>, Box<dyn Error>>;
  async fn get_by_id(&self, id: u32) -> Result<Option<User>, Box<dyn Error>>;
  async fn get_by_ids(&self, ids: &[i32]) -> Result<Vec<User>, Box<dyn Error>>;
  async fn get_by_name(&self, name: &str, pagination: Pagination) -> Result<ItemsTotal<User>, Box<dyn Error>>;
}
