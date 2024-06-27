use std::error::Error;
use async_trait::async_trait;
use domain::entities::book::book_involved::InvolvedId;

#[async_trait]
pub trait MutBookInvolvedRepository: Send + Sync {
  async fn add(&self, book_id: u32, involved: &[InvolvedId]) -> Result<(), Box<dyn Error>>;
  async fn remove(&self, book_id: u32, involved: &[InvolvedId]) -> Result<(), Box<dyn Error>>;
  async fn remove_all(&self, book_ids: &[u32]) -> Result<(), Box<dyn Error>>;
}
