pub mod mut_book_involved_repository;

use std::error::Error;

use async_trait::async_trait;

use domain::entities::book::book_involved::BookInvolved;
use domain::entities::involved::InvolvedId;
use domain::enums::language::Language;

#[async_trait]
pub trait BookInvolvedRepository: Send + Sync {
  async fn get(&self, book_id: u32, languages: &[Language]) -> Result<Vec<BookInvolved>, Box<dyn Error>>;
  async fn filter_existing(&self, book_id: u32, involved: &[InvolvedId]) -> Result<Vec<InvolvedId>, Box<dyn Error>>;
}
