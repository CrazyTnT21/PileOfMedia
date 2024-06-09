use std::sync::Arc;
use async_trait::async_trait;

use domain::entities::user::create_user::CreateUser;
use domain::entities::user::partial_create_user::PartialCreateUser;
use domain::entities::user::User;
use repositories::user_repository::mut_user_repository::MutUserRepository;
use services::image_service::mut_image_service::MutImageService;
use services::traits::service_error::ServiceError;
use services::user_service::mut_user_service::MutUserService;

use crate::services::map_server_error;

pub struct DefaultMutUserService<'a> {
  mut_user_repository: Arc<dyn MutUserRepository + 'a>,
  mut_image_service: Arc<dyn MutImageService + 'a>,
}

impl<'a> DefaultMutUserService<'a> {
  pub fn new(mut_user_repository: Arc<dyn MutUserRepository + 'a>,
             mut_image_service: Arc<dyn MutImageService + 'a>, ) -> DefaultMutUserService<'a> {
    DefaultMutUserService { mut_user_repository, mut_image_service }
  }
}

#[async_trait]
impl<'a> MutUserService for DefaultMutUserService<'a> {
  async fn create(&self, user: PartialCreateUser) -> Result<User, ServiceError> {
    let image = match user.profile_picture {
      None => None,
      Some(value) => Some(self.mut_image_service.create(value).await?)
    };
    let user = CreateUser {
      name: user.name,
      description: user.description,
      profile_picture: image,
    };
    self.mut_user_repository.create(user).await.map_err(map_server_error)
  }
}
