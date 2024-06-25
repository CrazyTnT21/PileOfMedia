use std::sync::Arc;
use async_trait::async_trait;

use domain::entities::role::Role;
use domain::enums::language::Language;
use domain::items_total::ItemsTotal;
use domain::pagination::Pagination;
use repositories::role_repository::RoleRepository;
use services::role_service::RoleService;
use services::traits::service_error::ServiceError;

use crate::services::map_server_error;

pub struct DefaultRoleService<'a> {
  role_repository: Arc<dyn RoleRepository + 'a>,
}

impl<'a> DefaultRoleService<'a> {
  pub fn new(role_repository: Arc<dyn RoleRepository + 'a>) -> DefaultRoleService<'a> {
    DefaultRoleService { role_repository }
  }
}

#[async_trait]
impl<'a> RoleService for DefaultRoleService<'a> {
  async fn get(&self, language: Language, pagination: Pagination) -> Result<ItemsTotal<Role>, ServiceError> {
    self.role_repository.get(language, pagination).await.map_err(map_server_error)
  }

  async fn get_by_id(&self, id: u32, language: Language) -> Result<Option<Role>, ServiceError> {
    self.role_repository.get_by_id(id, language).await.map_err(map_server_error)
  }

  async fn get_by_name(&self, name: &str, language: Language, pagination: Pagination) -> Result<ItemsTotal<Role>, ServiceError> {
    self.role_repository.get_by_name(name, language, pagination).await.map_err(map_server_error)
  }
}
