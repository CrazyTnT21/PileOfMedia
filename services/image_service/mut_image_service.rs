use std::error::Error;
use std::fmt::{Display, Formatter};

use async_trait::async_trait;

use crate::traits::service_error::ServiceError;
use domain::entities::image::Image;
use domain::entities::image::create_image::CreateImage;

#[async_trait]
pub trait MutImageService: Send + Sync {
  async fn create(&self, image: CreateImage) -> Result<Image, ServiceError<MutImageServiceError>>;
}

#[derive(Debug)]
pub enum MutImageServiceError {
  OtherError(Box<dyn Error>),
}

impl Display for MutImageServiceError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}",
      match self {
        MutImageServiceError::OtherError(x) => x.to_string(),
      }
    )
  }
}

impl Error for MutImageServiceError {
  fn source(&self) -> Option<&(dyn Error + 'static)> {
    match self {
      MutImageServiceError::OtherError(error) => Some(&**error),
    }
  }
}
