use std::sync::Arc;

use async_trait::async_trait;

use domain::entities::image::create_image::CreateImage;
use domain::entities::image::create_partial_image::CreatePartialImage;
use domain::entities::image::Image;
use repositories::image_repository::mut_image_repository::MutImageRepository;
use services::file_service::mut_file_service::MutFileService;
use services::image_service::mut_image_service::MutImageServiceError::OtherError;
use services::image_service::mut_image_service::{MutImageService, MutImageServiceError};
use services::traits::service_error::ServiceError;
use services::traits::service_error::ServiceError::ClientError;

pub struct DefaultMutImageService<'a> {
  mut_image_repository: Arc<dyn MutImageRepository + 'a>,
  mut_file_service: Arc<dyn MutFileService + 'a>,
  display_path: &'a str,
  path: &'a str,
}

impl<'a> DefaultMutImageService<'a> {
  pub fn new(
    mut_image_repository: Arc<dyn MutImageRepository + 'a>,
    mut_file_service: Arc<dyn MutFileService + 'a>,
    display_path: &'a str,
    path: &'a str,
  ) -> DefaultMutImageService<'a> {
    DefaultMutImageService {
      mut_image_repository,
      mut_file_service,
      display_path,
      path,
    }
  }
}

#[async_trait]
impl<'a> MutImageService for DefaultMutImageService<'a> {
  async fn create(&self, image: CreateImage) -> Result<Image, ServiceError<MutImageServiceError>> {
    //TODO: Validate data size
    let file = self
      .mut_file_service
      .create(&image.0, self.path, None)
      .await
      .map_err(|x| match x {
        ClientError(x) => ClientError(OtherError(Box::new(x))),
        ServiceError::ServerError(x) => ServiceError::ServerError(x),
      })?;
    let image = CreatePartialImage {
      file_path: self.path,
      uri: &file.uri,
      file_name: &file.name,
      display_path: self.display_path,
    };
    Ok(self.mut_image_repository.create(image).await?)
  }
}
