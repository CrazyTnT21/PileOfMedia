use std::error::Error;

use async_trait::async_trait;

use domain::entities::image::Image;
use domain::entities::image::create_partial_image::CreatePartialImage;

#[async_trait]
pub trait MutImageRepository: Send + Sync {
  async fn create(&self, image: CreatePartialImage<'_>) -> Result<Image, Box<dyn Error>>;
}
