use async_trait::async_trait;

use domain::entities::user::create_user::CreateUser;
use domain::entities::user::User;

use crate::traits::service_error::ServiceError;

#[async_trait]
pub trait MutUserService: Send + Sync {
  async fn create(&self, user: CreateUser) -> Result<User, ServiceError>;
}
