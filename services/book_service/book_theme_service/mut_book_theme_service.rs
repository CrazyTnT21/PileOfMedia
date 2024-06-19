use async_trait::async_trait;

use crate::traits::service_error::ServiceError;

#[async_trait]
pub trait MutBookThemeService: Send + Sync {
  async fn add(&self, book_id: u32, themes: &[u32]) -> Result<(), ServiceError>;
  async fn remove(&self, book_id: u32, themes: &[u32]) -> Result<(), ServiceError>;
}
