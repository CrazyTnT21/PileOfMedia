pub mod mut_image_service;

use async_trait::async_trait;

use domain::entities::image::Image;
use domain::items_total::ItemsTotal;
use domain::pagination::Pagination;

use crate::traits::service_error::ServiceError;

#[async_trait]
pub trait ImageService: Send + Sync {
  async fn get(&self, pagination: Pagination) -> Result<ItemsTotal<Image>, ServiceError>;
  async fn get_by_id(&self, id: u32) -> Result<Option<Image>, ServiceError>;
}
