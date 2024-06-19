use std::error::Error;
use async_trait::async_trait;

#[async_trait]
pub trait MutBookGenreRepository: Send + Sync {
  async fn add(&self, book_id: u32, genres: &[u32]) -> Result<(), Box<dyn Error>>;
  async fn remove(&self, book_id: u32, genres: &[u32]) -> Result<(), Box<dyn Error>>;
}
