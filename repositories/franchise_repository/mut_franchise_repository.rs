use async_trait::async_trait;
use domain::entities::franchise::create_partial_franchise::CreatePartialFranchise;
use domain::entities::franchise::Franchise;
use std::error::Error;

#[async_trait]
pub trait MutFranchiseRepository: Send + Sync {
  async fn create(&self, item: CreatePartialFranchise) -> Result<Franchise, Box<dyn Error>>;
  async fn delete(&self, ids: &[u32]) -> Result<(), Box<dyn Error>>;
}
