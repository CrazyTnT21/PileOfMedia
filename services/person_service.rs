use async_trait::async_trait;

use domain::entities::person::Person;
use domain::enums::language::Language;
use domain::items_total::ItemsTotal;
use domain::pagination::Pagination;

use crate::traits::service_error::ServiceError;

#[async_trait]
pub trait PersonService: Send + Sync {
  async fn get(&self, language: Language, pagination: Pagination) -> Result<ItemsTotal<Person>, ServiceError>;
  async fn get_by_id(&self, id: u32, language: Language) -> Result<Option<Person>, ServiceError>;
  async fn get_by_ids(&self, ids: &[u32], language: Language) -> Result<Vec<Person>, ServiceError>;
  async fn get_by_name(&self, name: &str, language: Language,pagination: Pagination) -> Result<ItemsTotal<Person>, ServiceError>;
}
