use std::sync::Arc;

use async_trait::async_trait;

use domain::entities::role::Role;
use domain::enums::language::Language;
use domain::items_total::ItemsTotal;
use domain::pagination::Pagination;
use repositories::role_repository::RoleRepository;
use services::role_service::{RoleService, RoleServiceError};
use services::traits::service_error::ServiceError;

pub struct DefaultRoleService<'a> {
  role_repository: Arc<dyn RoleRepository + 'a>,
}

impl<'a> DefaultRoleService<'a> {
  pub fn new(role_repository: Arc<dyn RoleRepository + 'a>) -> DefaultRoleService<'a> {
    DefaultRoleService { role_repository }
  }
}

#[async_trait]
impl RoleService for DefaultRoleService<'_> {
  async fn get(
    &self,
    languages: &[Language],
    pagination: Pagination,
  ) -> Result<ItemsTotal<Role>, ServiceError<RoleServiceError>> {
    Ok(self.role_repository.get(languages, pagination).await?)
  }

  async fn get_by_id(&self, id: u32, languages: &[Language]) -> Result<Option<Role>, ServiceError<RoleServiceError>> {
    Ok(self.role_repository.get_by_id(id, languages).await?)
  }

  async fn get_by_name(
    &self,
    name: &str,
    languages: &[Language],
    pagination: Pagination,
  ) -> Result<ItemsTotal<Role>, ServiceError<RoleServiceError>> {
    Ok(self.role_repository.get_by_name(name, languages, pagination).await?)
  }
}
