use std::sync::Arc;

use async_trait::async_trait;

use domain::entities::person::Person;
use domain::enums::language::Language;
use domain::items_total::ItemsTotal;
use domain::pagination::Pagination;
use repositories::person_repository::PersonRepository;
use services::person_service::{PersonService, PersonServiceError};
use services::traits::service_error::ServiceError;

pub struct DefaultPersonService<'a> {
  person_repository: Arc<dyn PersonRepository + 'a>,
}

impl<'a> DefaultPersonService<'a> {
  pub fn new(person_repository: Arc<dyn PersonRepository + 'a>) -> DefaultPersonService<'a> {
    DefaultPersonService { person_repository }
  }
}

#[async_trait]
impl PersonService for DefaultPersonService<'_> {
  async fn get(
    &self,
    languages: &[Language],
    pagination: Pagination,
  ) -> Result<ItemsTotal<Person>, ServiceError<PersonServiceError>> {
    Ok(self.person_repository.get(languages, pagination).await?)
  }

  async fn get_by_id(
    &self,
    id: u32,
    languages: &[Language],
  ) -> Result<Option<Person>, ServiceError<PersonServiceError>> {
    Ok(self.person_repository.get_by_id(id, languages).await?)
  }

  async fn get_by_name(
    &self,
    name: &str,
    languages: &[Language],
    pagination: Pagination,
  ) -> Result<ItemsTotal<Person>, ServiceError<PersonServiceError>> {
    Ok(self.person_repository.get_by_name(name, languages, pagination).await?)
  }
}
