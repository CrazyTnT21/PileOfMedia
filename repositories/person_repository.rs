pub mod mut_person_repository;

use std::error::Error;

use async_trait::async_trait;

use domain::entities::person::Person;
use domain::enums::language::Language;
use domain::items_total::ItemsTotal;
use domain::pagination::Pagination;

#[async_trait]
pub trait PersonRepository: Send + Sync {
  async fn get(&self, language: Language, pagination: Pagination) -> Result<ItemsTotal<Person>, Box<dyn Error>>;
  async fn get_by_id(&self, id: u32, language: Language) -> Result<Option<Person>, Box<dyn Error>>;
  async fn get_by_ids(&self, ids: &[u32], language: Language) -> Result<Vec<Person>, Box<dyn Error>>;
  async fn get_by_name(&self, name: &str, language: Language, pagination: Pagination) -> Result<ItemsTotal<Person>, Box<dyn Error>>;

  async fn filter_existing(&self, people: &[u32]) -> Result<Vec<u32>, Box<dyn Error>>;
}
