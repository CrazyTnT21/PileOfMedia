use async_trait::async_trait;
use domain::entities::book::book_involved::InvolvedId;

use crate::traits::service_error::ServiceError;

#[async_trait]
pub trait MutBookInvolvedService: Send + Sync {
  async fn add(&self, book_id: u32, involved: &[InvolvedId]) -> Result<(), ServiceError>;
  async fn remove(&self, book_id: u32, involved: &[InvolvedId]) -> Result<(), ServiceError>;
}
