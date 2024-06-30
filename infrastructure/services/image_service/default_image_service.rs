use std::sync::Arc;

use async_trait::async_trait;

use domain::entities::image::Image;
use domain::items_total::ItemsTotal;
use domain::pagination::Pagination;
use repositories::image_repository::ImageRepository;
use services::image_service::{ImageService, ImageServiceError};
use services::traits::service_error::ServiceError;

pub struct DefaultImageService<'a> {
  image_repository: Arc<dyn ImageRepository + 'a>,
}

impl<'a> DefaultImageService<'a> {
  pub fn new(image_repository: Arc<dyn ImageRepository + 'a>) -> DefaultImageService<'a> {
    DefaultImageService { image_repository }
  }
}

#[async_trait]
impl<'a> ImageService for DefaultImageService<'a> {
  async fn get(&self, pagination: Pagination) -> Result<ItemsTotal<Image>, ServiceError<ImageServiceError>> {
    Ok(self.image_repository.get(pagination).await?)
  }

  async fn get_by_id(&self, id: u32) -> Result<Option<Image>, ServiceError<ImageServiceError>> {
    Ok(self.image_repository.get_by_id(id).await?)
  }
}
