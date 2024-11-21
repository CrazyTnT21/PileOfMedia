use async_trait::async_trait;
use domain::entities::role::create_partial_role::CreatePartialRole;
use domain::entities::role::Role;
use std::error::Error;

#[async_trait]
pub trait MutRoleRepository: Send + Sync {
  async fn create(&self, item: CreatePartialRole) -> Result<Role, Box<dyn Error>>;
  async fn delete(&self, ids: &[u32]) -> Result<(), Box<dyn Error>>;
}
