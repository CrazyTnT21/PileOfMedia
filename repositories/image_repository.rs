use std::error::Error;

use async_trait::async_trait;

use domain::entities::image::Image;
use domain::items_total::ItemsTotal;
use domain::pagination::Pagination;

#[async_trait]
pub trait ImageRepository: Send + Sync {
  async fn get(&self, pagination: Pagination) -> Result<ItemsTotal<Image>, Box<dyn Error>>;
  async fn get_by_id(&self, id: u32) -> Result<Option<Image>, Box<dyn Error>>;
  async fn get_by_ids(&self, ids: &[i32]) -> Result<Vec<Image>, Box<dyn Error>>;
}
