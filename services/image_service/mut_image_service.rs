use async_trait::async_trait;
use domain::entities::image::Image;
use domain::entities::image::partial_create_image::PartialCreateImage;

use crate::traits::service_error::ServiceError;

#[async_trait]
pub trait MutImageService: Send + Sync {
  async fn create(&self, image: PartialCreateImage) -> Result<Image, ServiceError>;
}
