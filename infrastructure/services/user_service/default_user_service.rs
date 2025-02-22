use std::sync::Arc;

use async_trait::async_trait;

use domain::entities::user::User;
use domain::items_total::ItemsTotal;
use domain::pagination::Pagination;
use repositories::user_repository::UserRepository;
use services::traits::service_error::ServiceError;
use services::user_service::{UserService, UserServiceError};

pub struct DefaultUserService<'a> {
  user_repository: Arc<dyn UserRepository + 'a>,
}

impl<'a> DefaultUserService<'a> {
  pub fn new(user_repository: Arc<dyn UserRepository + 'a>) -> DefaultUserService<'a> {
    DefaultUserService { user_repository }
  }
}

#[async_trait]
impl UserService for DefaultUserService<'_> {
  async fn get(&self, pagination: Pagination) -> Result<ItemsTotal<User>, ServiceError<UserServiceError>> {
    Ok(self.user_repository.get(pagination).await?)
  }

  async fn get_by_id(&self, id: u32) -> Result<Option<User>, ServiceError<UserServiceError>> {
    Ok(self.user_repository.get_by_id(id).await?)
  }

  async fn get_by_name(
    &self,
    name: &str,
    pagination: Pagination,
  ) -> Result<ItemsTotal<User>, ServiceError<UserServiceError>> {
    Ok(self.user_repository.get_by_name(name, pagination).await?)
  }

  async fn get_by_username(&self, name: &str) -> Result<Option<User>, ServiceError<UserServiceError>> {
    Ok(self.user_repository.get_by_username(name).await?)
  }
}
