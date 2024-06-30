pub mod mut_franchise_repository;

use std::error::Error;

use async_trait::async_trait;

use domain::entities::franchise::Franchise;
use domain::enums::language::Language;
use domain::items_total::ItemsTotal;
use domain::pagination::Pagination;

#[async_trait]
pub trait FranchiseRepository: Send + Sync {
  async fn get(&self, language: Language, pagination: Pagination) -> Result<ItemsTotal<Franchise>, Box<dyn Error>>;
  async fn get_by_id(&self, id: u32, language: Language) -> Result<Option<Franchise>, Box<dyn Error>>;
  async fn get_by_ids(&self, ids: &[u32], language: Language) -> Result<Vec<Franchise>, Box<dyn Error>>;
  async fn get_by_name(&self, name: &str, language: Language, pagination: Pagination) -> Result<ItemsTotal<Franchise>, Box<dyn Error>>;

  async fn filter_existing(&self, franchises: &[u32]) -> Result<Vec<u32>, Box<dyn Error>>;
}
