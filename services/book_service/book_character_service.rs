pub mod mut_book_character_service;

use async_trait::async_trait;

use domain::entities::book::book_character::BookCharacter;
use domain::enums::language::Language;
use domain::items_total::ItemsTotal;
use domain::pagination::Pagination;

use crate::traits::service_error::ServiceError;

#[async_trait]
pub trait BookCharacterService: Send + Sync {
  async fn get(&self, book_id: u32, language: Language, pagination: Pagination) -> Result<ItemsTotal<BookCharacter>, ServiceError>;
}
