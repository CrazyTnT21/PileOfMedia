use std::sync::Arc;
use async_trait::async_trait;

use domain::entities::character::Character;
use domain::enums::language::Language;
use domain::items_total::ItemsTotal;
use domain::pagination::Pagination;
use repositories::character_repository::CharacterRepository;
use services::character_service::CharacterService;
use services::traits::service_error::ServiceError;

use crate::services::map_server_error;

pub struct DefaultCharacterService<'a> {
  character_repository: Arc<dyn CharacterRepository + 'a>,
}

impl<'a> DefaultCharacterService<'a> {
  pub fn new(character_repository: Arc<dyn CharacterRepository + 'a>) -> DefaultCharacterService<'a> {
    DefaultCharacterService { character_repository }
  }
}

#[async_trait]
impl<'a> CharacterService for DefaultCharacterService<'a> {
  async fn get(&self, language: Language, pagination: Pagination) -> Result<ItemsTotal<Character>, ServiceError> {
    self.character_repository.get(language, pagination).await.map_err(map_server_error)
  }

  async fn get_by_id(&self, id: u32, language: Language) -> Result<Option<Character>, ServiceError> {
    self.character_repository.get_by_id(id, language).await.map_err(map_server_error)
  }

  async fn get_by_ids(&self, ids: &[u32], language: Language) -> Result<Vec<Character>, ServiceError> {
    self.character_repository.get_by_ids(ids, language).await.map_err(map_server_error)
  }

  async fn get_by_name(&self, name: &str, language: Language, pagination: Pagination) -> Result<ItemsTotal<Character>, ServiceError> {
    self.character_repository.get_by_name(name, language, pagination).await.map_err(map_server_error)
  }
}
