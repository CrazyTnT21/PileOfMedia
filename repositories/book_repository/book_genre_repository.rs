pub mod mut_book_genre_repository;

use std::collections::HashMap;
use std::error::Error;

use async_trait::async_trait;

use domain::entities::genre::Genre;
use domain::enums::language::Language;

#[async_trait]
pub trait BookGenreRepository: Send + Sync {
  async fn get_by_id(&self, book_id: u32, languages: &[Language]) -> Result<Vec<Genre>, Box<dyn Error>>;
  async fn get_by_ids(
    &self,
    book_ids: &[u32],
    languages: &[Language],
  ) -> Result<HashMap<u32, Vec<Genre>>, Box<dyn Error>>;
  async fn filter_existing(&self, book_id: u32, genres: &[u32]) -> Result<Vec<u32>, Box<dyn Error>>;
}
