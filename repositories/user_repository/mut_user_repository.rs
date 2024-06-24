use std::error::Error;

use async_trait::async_trait;

use domain::entities::user::create_partial_user::CreatePartialUser;
use domain::entities::user::User;

#[async_trait]
pub trait MutUserRepository: Send + Sync {
  async fn create(&self, user: CreatePartialUser) -> Result<User, Box<dyn Error>>;
}
