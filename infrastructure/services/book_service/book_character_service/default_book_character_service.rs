use std::sync::Arc;

use async_trait::async_trait;

use domain::entities::book::book_character::BookCharacter;
use domain::enums::language::Language;
use domain::items_total::ItemsTotal;
use domain::pagination::Pagination;
use repositories::book_repository::book_character_repository::BookCharacterRepository;
use services::book_service::book_character_service::{BookCharacterService, BookCharacterServiceError};
use services::traits::service_error::ServiceError;

pub struct DefaultBookCharacterService<'a> {
  book_character_repository: Arc<dyn BookCharacterRepository + 'a>,
}

impl<'a> DefaultBookCharacterService<'a> {
  pub fn new(book_character_repository: Arc<dyn BookCharacterRepository + 'a>) -> DefaultBookCharacterService<'a> {
    DefaultBookCharacterService {
      book_character_repository,
    }
  }
}

#[async_trait]
impl<'a> BookCharacterService for DefaultBookCharacterService<'a> {
  async fn get(
    &self,
    book_id: u32,
    language: Language,
    pagination: Pagination,
  ) -> Result<ItemsTotal<BookCharacter>, ServiceError<BookCharacterServiceError>> {
    Ok(
      self
        .book_character_repository
        .get(book_id, language, pagination)
        .await?,
    )
  }
}
