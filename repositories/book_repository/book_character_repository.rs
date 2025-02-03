pub mod mut_book_character_repository;

use std::error::Error;

use async_trait::async_trait;
use domain::entities::book::book_character::BookCharacter;
use domain::enums::language::Language;

#[async_trait]
pub trait BookCharacterRepository: Send + Sync {
  async fn get(&self, book_id: u32, languages: &[Language]) -> Result<Vec<BookCharacter>, Box<dyn Error>>;
  async fn filter_existing(&self, book_id: u32, characters: &[u32]) -> Result<Vec<u32>, Box<dyn Error>>;
}
