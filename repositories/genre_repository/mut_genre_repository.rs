use async_trait::async_trait;
use domain::entities::genre::create_partial_genre::CreatePartialGenre;
use domain::entities::genre::Genre;
use std::error::Error;

#[async_trait]
pub trait MutGenreRepository: Send + Sync {
  async fn create(&self, item: CreatePartialGenre) -> Result<Genre, Box<dyn Error>>;
  async fn delete(&self, ids: &[u32]) -> Result<(), Box<dyn Error>>;
}
