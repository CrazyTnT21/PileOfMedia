use std::error::Error;

use async_trait::async_trait;

use domain::entities::character::create_partial_character::CreatePartialCharacter;
use domain::entities::character::Character;

#[async_trait]
pub trait MutCharacterRepository: Send + Sync {
  async fn create(&self, item: CreatePartialCharacter) -> Result<Character, Box<dyn Error>>;
  async fn delete(&self, ids: &[u32]) -> Result<(), Box<dyn Error>>;
}
