use std::error::Error;
use std::sync::Arc;

use async_trait::async_trait;
use tokio_postgres::Transaction;

use domain::entities::user::User;
use domain::entities::user::create_partial_user::CreatePartialUser;
use repositories::user_repository::UserRepository;
use repositories::user_repository::mut_user_repository::MutUserRepository;

use crate::insert::Insert;
use crate::schemas::db_user::DbUser;

pub struct DefaultMutUserRepository<'a> {
  transaction: &'a Transaction<'a>,
  user_repository: Arc<dyn UserRepository + 'a>,
}

impl<'a> DefaultMutUserRepository<'a> {
  pub fn new(
    transaction: &'a Transaction<'a>,
    user_repository: Arc<dyn UserRepository + 'a>,
  ) -> DefaultMutUserRepository<'a> {
    DefaultMutUserRepository {
      transaction,
      user_repository,
    }
  }
}

#[async_trait]
impl MutUserRepository for DefaultMutUserRepository<'_> {
  async fn create(&self, user: CreatePartialUser) -> Result<User, Box<dyn Error>> {
    let id: i32 = Insert::new::<DbUser>(["name", "description", "fkprofilepicture"])
      .values([
        &user.name,
        &user.description,
        &user.profile_picture.map(|x| x.id as i32),
      ])
      .returning_transaction("id", self.transaction)
      .await?;
    Ok(
      self
        .user_repository
        .get_by_id(id as u32)
        .await?
        .expect("User was just created, they should exist"),
    )
  }
}
