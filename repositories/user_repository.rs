use std::error::Error;

use async_trait::async_trait;

use domain::entities::user::User;
use domain::items_total::ItemsTotal;
use domain::pagination::Pagination;

pub mod mut_user_repository;
pub mod user_book_repository;

#[async_trait]
pub trait UserRepository: Send + Sync {
  async fn get(&self, pagination: Pagination) -> Result<ItemsTotal<User>, Box<dyn Error>>;
  async fn get_by_id(&self, id: u32) -> Result<Option<User>, Box<dyn Error>>;
  async fn get_by_ids(&self, ids: &[u32]) -> Result<Vec<User>, Box<dyn Error>>;
  async fn get_by_name(&self, name: &str, pagination: Pagination) -> Result<ItemsTotal<User>, Box<dyn Error>>;
  async fn get_by_username(&self, name: &str) -> Result<Option<User>, Box<dyn Error>>;

  async fn filter_existing(&self, users: &[u32]) -> Result<Vec<u32>, Box<dyn Error>>;
}
