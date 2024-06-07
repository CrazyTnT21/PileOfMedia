use std::error::Error;

use async_trait::async_trait;

use domain::entities::user::create_user::CreateUser;
use domain::entities::user::User;

#[async_trait]
pub trait MutUserRepository: Send + Sync {
  async fn create(&self, user: CreateUser) -> Result<User, Box<dyn Error>>;
}
