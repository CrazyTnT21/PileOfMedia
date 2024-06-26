use std::fmt::{Display, Formatter};

use async_trait::async_trait;

use domain::entities::role::Role;
use domain::enums::language::Language;
use domain::items_total::ItemsTotal;
use domain::pagination::Pagination;

use crate::traits::service_error::ServiceError;

#[async_trait]
pub trait RoleService: Send + Sync {
  async fn get(&self, language: Language, pagination: Pagination) -> Result<ItemsTotal<Role>, ServiceError<RoleServiceError>>;
  async fn get_by_id(&self, id: u32, language: Language) -> Result<Option<Role>, ServiceError<RoleServiceError>>;
  async fn get_by_name(&self, name: &str, language: Language, pagination: Pagination) -> Result<ItemsTotal<Role>, ServiceError<RoleServiceError>>;
}

pub enum RoleServiceError {}

impl Display for RoleServiceError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "")
  }
}
