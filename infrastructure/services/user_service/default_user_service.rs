use std::sync::Arc;
use async_trait::async_trait;

use domain::entities::user::User;
use domain::items_total::ItemsTotal;
use domain::pagination::Pagination;
use repositories::user_repository::UserRepository;
use services::traits::service_error::ServiceError;
use services::user_service::UserService;

use crate::services::map_server_error;

pub struct DefaultUserService<'a> {
  user_repository: Arc<dyn UserRepository + 'a>,
}

impl<'a> DefaultUserService<'a> {
  pub fn new(user_repository: Arc<dyn UserRepository + 'a>) -> DefaultUserService<'a> {
    DefaultUserService { user_repository }
  }
}

#[async_trait]
impl<'a> UserService for DefaultUserService<'a> {
  async fn get(&self, pagination: Pagination) -> Result<ItemsTotal<User>, ServiceError> {
    self.user_repository.get(pagination).await.map_err(map_server_error)
  }

  async fn get_by_id(&self, id: u32) -> Result<Option<User>, ServiceError> {
    self.user_repository.get_by_id(id).await.map_err(map_server_error)
  }

  async fn get_by_name(&self, name: &str, pagination: Pagination) -> Result<ItemsTotal<User>, ServiceError> {
    self.user_repository.get_by_name(name, pagination).await.map_err(map_server_error)
  }
}
