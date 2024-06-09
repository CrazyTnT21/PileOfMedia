use std::sync::Arc;
use async_trait::async_trait;

use domain::entities::person::Person;
use domain::enums::language::Language;
use domain::items_total::ItemsTotal;
use domain::pagination::Pagination;
use repositories::person_repository::PersonRepository;
use services::person_service::PersonService;
use services::traits::service_error::ServiceError;

use crate::services::map_server_error;

pub struct DefaultPersonService<'a> {
  person_repository: Arc<dyn PersonRepository + 'a>,
}

impl<'a> DefaultPersonService<'a> {
  pub fn new(person_repository: Arc<dyn PersonRepository + 'a>) -> DefaultPersonService {
    DefaultPersonService { person_repository }
  }
}

#[async_trait]
impl<'a> PersonService for DefaultPersonService<'a> {
  async fn get(&self, language: Language, pagination: Pagination) -> Result<ItemsTotal<Person>, ServiceError> {
    self.person_repository.get(language, pagination).await.map_err(map_server_error)
  }

  async fn get_by_id(&self, id: u32, language: Language) -> Result<Option<Person>, ServiceError> {
    self.person_repository.get_by_id(id, language).await.map_err(map_server_error)
  }

  async fn get_by_ids(&self, ids: &[i32], language: Language) -> Result<Vec<Person>, ServiceError> {
    self.person_repository.get_by_ids(ids, language).await.map_err(map_server_error)
  }

  async fn get_by_name(&self, name: &str, language: Language, pagination: Pagination) -> Result<ItemsTotal<Person>, ServiceError> {
    self.person_repository.get_by_name(name, language, pagination).await.map_err(map_server_error)
  }
}
