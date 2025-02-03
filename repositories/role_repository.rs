pub mod mut_role_repository;

use std::error::Error;

use async_trait::async_trait;

use domain::entities::role::Role;
use domain::enums::language::Language;
use domain::items_total::ItemsTotal;
use domain::pagination::Pagination;

#[async_trait]
pub trait RoleRepository: Send + Sync {
  async fn get(&self, languages: &[Language], pagination: Pagination) -> Result<ItemsTotal<Role>, Box<dyn Error>>;
  async fn get_by_id(&self, id: u32, languages: &[Language]) -> Result<Option<Role>, Box<dyn Error>>;
  async fn get_by_ids(&self, ids: &[u32], languages: &[Language]) -> Result<Vec<Role>, Box<dyn Error>>;
  async fn get_by_name(
    &self,
    name: &str,
    languages: &[Language],
    pagination: Pagination,
  ) -> Result<ItemsTotal<Role>, Box<dyn Error>>;

  async fn filter_existing(&self, roles: &[u32]) -> Result<Vec<u32>, Box<dyn Error>>;
}
