pub mod mut_character_service;

use std::error::Error;
use std::fmt::{Display, Formatter};

use async_trait::async_trait;

use crate::traits::service_error::ServiceError;
use domain::entities::character::Character;
use domain::enums::language::Language;
use domain::items_total::ItemsTotal;
use domain::pagination::Pagination;

#[async_trait]
pub trait CharacterService: Send + Sync {
  async fn get(
    &self,
    languages: &[Language],
    pagination: Pagination,
  ) -> Result<ItemsTotal<Character>, ServiceError<CharacterServiceError>>;
  async fn get_by_id(
    &self,
    id: u32,
    languages: &[Language],
  ) -> Result<Option<Character>, ServiceError<CharacterServiceError>>;
  async fn get_by_name(
    &self,
    name: &str,
    languages: &[Language],
    pagination: Pagination,
  ) -> Result<ItemsTotal<Character>, ServiceError<CharacterServiceError>>;
}
#[derive(Debug)]
pub enum CharacterServiceError {
  OtherError(Box<dyn Error>),
}

impl Display for CharacterServiceError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}",
      match self {
        CharacterServiceError::OtherError(value) => value.to_string(),
      }
    )
  }
}

impl Error for CharacterServiceError {
  fn source(&self) -> Option<&(dyn Error + 'static)> {
    match self {
      CharacterServiceError::OtherError(error) => Some(&**error),
    }
  }
}
