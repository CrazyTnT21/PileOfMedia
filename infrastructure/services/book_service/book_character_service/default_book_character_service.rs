use std::sync::Arc;

use async_trait::async_trait;

use domain::entities::book::book_character::BookCharacter;
use domain::enums::language::Language;
use domain::items_total::ItemsTotal;
use domain::pagination::Pagination;
use repositories::book_repository::book_character_repository::BookCharacterRepository;
use services::book_service::book_character_service::BookCharacterService;
use services::traits::service_error::ServiceError;

use crate::services::map_server_error;

pub struct DefaultBookCharacterService<'a> {
  book_character_repository: Arc<dyn BookCharacterRepository + 'a>,
}

impl<'a> DefaultBookCharacterService<'a> {
  pub fn new(book_character_repository: Arc<dyn BookCharacterRepository + 'a>) -> DefaultBookCharacterService<'a> {
    DefaultBookCharacterService { book_character_repository }
  }
}

#[async_trait]
impl<'a> BookCharacterService for DefaultBookCharacterService<'a> {
  async fn get(&self, book_id: u32, language: Language, pagination: Pagination) -> Result<ItemsTotal<BookCharacter>, ServiceError> {
    self.book_character_repository.get(book_id, language, pagination).await.map_err(map_server_error)
  }
}
