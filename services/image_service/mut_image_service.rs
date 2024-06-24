use async_trait::async_trait;
use domain::entities::image::Image;
use domain::entities::image::create_image::CreateImage;

use crate::traits::service_error::ServiceError;

#[async_trait]
pub trait MutImageService: Send + Sync {
  async fn create(&self, image: CreateImage) -> Result<Image, ServiceError>;
}
