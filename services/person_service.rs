pub mod mut_person_service;

use std::error::Error;
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

#[derive(Debug)]
pub enum PersonServiceError {
  OtherError(Box<dyn Error>),
}

impl Display for PersonServiceError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}",
      match self {
        PersonServiceError::OtherError(value) => value.to_string(),
      }
    )
  }
}

impl Error for PersonServiceError {
  fn source(&self) -> Option<&(dyn Error + 'static)> {
    match self {
      PersonServiceError::OtherError(error) => Some(&**error),
    }
  }
}
