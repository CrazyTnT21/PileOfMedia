use std::fmt::{Display, Formatter};

use async_trait::async_trait;

use domain::entities::image::create_image::CreateImage;
use domain::entities::image::Image;

use crate::traits::service_error::ServiceError;

#[async_trait]
pub trait MutImageService: Send + Sync {
  async fn create(&self, image: CreateImage) -> Result<Image, ServiceError<MutImageServiceError>>;
}

pub enum MutImageServiceError {
  OtherError(Box<dyn Display>),
}

impl Display for MutImageServiceError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", match self {
      MutImageServiceError::OtherError(x) => x.to_string()
    })
  }
}
