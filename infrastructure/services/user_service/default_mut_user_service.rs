use std::sync::Arc;

use async_trait::async_trait;

use domain::entities::user::User;
use domain::entities::user::create_partial_user::CreatePartialUser;
use domain::entities::user::create_user::CreateUser;
use repositories::user_repository::mut_user_repository::MutUserRepository;
use services::image_service::mut_image_service::MutImageService;
use services::traits::service_error::ServiceError;
use services::traits::service_error::ServiceError::ClientError;
use services::user_service::mut_user_service::MutUserServiceError::OtherError;
use services::user_service::mut_user_service::{MutUserService, MutUserServiceError};

pub struct DefaultMutUserService<'a> {
  mut_user_repository: Arc<dyn MutUserRepository + 'a>,
  mut_image_service: Arc<dyn MutImageService + 'a>,
}

impl<'a> DefaultMutUserService<'a> {
  pub fn new(
    mut_user_repository: Arc<dyn MutUserRepository + 'a>,
    mut_image_service: Arc<dyn MutImageService + 'a>,
  ) -> DefaultMutUserService<'a> {
    DefaultMutUserService {
      mut_user_repository,
      mut_image_service,
    }
  }
}

#[async_trait]
impl MutUserService for DefaultMutUserService<'_> {
  async fn create(&self, user: CreateUser) -> Result<User, ServiceError<MutUserServiceError>> {
    let data = user.user;
    let image = match user.profile_picture {
      None => None,
      Some(value) => Some(self.mut_image_service.create(value).await.map_err(|x| match x {
        ClientError(x) => ClientError(OtherError(Box::new(x))),
        ServiceError::ServerError(x) => ServiceError::ServerError(x),
      })?),
    };
    let user = CreatePartialUser {
      name: data.name,
      description: data.description,
      profile_picture: image,
    };
    Ok(self.mut_user_repository.create(user).await?)
  }
}
