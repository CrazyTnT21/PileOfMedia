use std::sync::Arc;

use async_trait::async_trait;

use domain::entities::character::Character;
use domain::enums::language::Language;
use domain::items_total::ItemsTotal;
use domain::pagination::Pagination;
use repositories::character_repository::CharacterRepository;
use services::character_service::{CharacterService, CharacterServiceError};
use services::traits::service_error::ServiceError;

pub struct DefaultCharacterService<'a> {
  character_repository: Arc<dyn CharacterRepository + 'a>,
}

impl<'a> DefaultCharacterService<'a> {
  pub fn new(character_repository: Arc<dyn CharacterRepository + 'a>) -> DefaultCharacterService<'a> {
    DefaultCharacterService { character_repository }
  }
}

#[async_trait]
impl CharacterService for DefaultCharacterService<'_> {
  async fn get(
    &self,
    language: Language,
    pagination: Pagination,
  ) -> Result<ItemsTotal<Character>, ServiceError<CharacterServiceError>> {
    Ok(self.character_repository.get(language, pagination).await?)
  }

  async fn get_by_id(
    &self,
    id: u32,
    language: Language,
  ) -> Result<Option<Character>, ServiceError<CharacterServiceError>> {
    Ok(self.character_repository.get_by_id(id, language).await?)
  }

  async fn get_by_name(
    &self,
    name: &str,
    language: Language,
    pagination: Pagination,
  ) -> Result<ItemsTotal<Character>, ServiceError<CharacterServiceError>> {
    Ok(
      self
        .character_repository
        .get_by_name(name, language, pagination)
        .await?,
    )
  }
}
