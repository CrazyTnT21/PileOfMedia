use std::error::Error;

use async_trait::async_trait;

use domain::entities::character::Character;
use domain::enums::language::Language;
use domain::items_total::ItemsTotal;
use domain::pagination::Pagination;

#[async_trait]
pub trait CharacterRepository: Send + Sync {
  async fn get(&self, language: Language, pagination: Pagination) -> Result<ItemsTotal<Character>, Box<dyn Error>>;
  async fn get_by_id(&self, id: u32, language: Language) -> Result<Option<Character>, Box<dyn Error>>;
  async fn get_by_ids(&self, ids: &[i32], language: Language) -> Result<Vec<Character>, Box<dyn Error>>;
  async fn get_by_name(&self, name: &str, language: Language,pagination: Pagination) -> Result<ItemsTotal<Character>, Box<dyn Error>>;
}
