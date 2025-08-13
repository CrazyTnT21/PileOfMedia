pub mod mut_role_service;

use std::error::Error;
use std::fmt::{Display, Formatter};

use async_trait::async_trait;

use domain::entities::role::Role;
use domain::enums::language::Language;
use domain::items_total::ItemsTotal;
use domain::pagination::Pagination;

use crate::traits::service_error::ServiceError;

#[async_trait]
pub trait RoleService: Send + Sync {
  async fn get(
    &self,
    languages: &[Language],
    pagination: Pagination,
  ) -> Result<ItemsTotal<Role>, ServiceError<RoleServiceError>>;
  async fn get_by_id(&self, id: u32, languages: &[Language]) -> Result<Option<Role>, ServiceError<RoleServiceError>>;
  async fn get_by_name(
    &self,
    name: &str,
    languages: &[Language],
    pagination: Pagination,
  ) -> Result<ItemsTotal<Role>, ServiceError<RoleServiceError>>;
}

#[derive(Debug)]
pub enum RoleServiceError {
  OtherError(Box<dyn Error>),
}

impl Display for RoleServiceError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}",
      match self {
        RoleServiceError::OtherError(value) => value.to_string(),
      }
    )
  }
}

impl Error for RoleServiceError {
  fn source(&self) -> Option<&(dyn Error + 'static)> {
    match self {
      RoleServiceError::OtherError(error) => Some(&**error),
    }
  }
}
