pub mod mut_person_service;

use std::fmt::{Display, Formatter};

use async_trait::async_trait;

use domain::entities::person::Person;
use domain::enums::language::Language;
use domain::items_total::ItemsTotal;
use domain::pagination::Pagination;

use crate::traits::service_error::ServiceError;

#[async_trait]
pub trait PersonService: Send + Sync {
  async fn get(
    &self,
    languages: &[Language],
    pagination: Pagination,
  ) -> Result<ItemsTotal<Person>, ServiceError<PersonServiceError>>;
  async fn get_by_id(
    &self,
    id: u32,
    languages: &[Language],
  ) -> Result<Option<Person>, ServiceError<PersonServiceError>>;
  async fn get_by_name(
    &self,
    name: &str,
    languages: &[Language],
    pagination: Pagination,
  ) -> Result<ItemsTotal<Person>, ServiceError<PersonServiceError>>;
}

pub enum PersonServiceError {}

impl Display for PersonServiceError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "")
  }
}
