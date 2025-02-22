use std::error::Error;

use async_trait::async_trait;

use domain::entities::user::User;
use domain::entities::user::create_partial_user::CreatePartialUser;

#[async_trait]
pub trait MutUserRepository: Send + Sync {
  async fn create(&self, user: CreatePartialUser) -> Result<User, Box<dyn Error>>;
}
