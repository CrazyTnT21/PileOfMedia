use std::error::Error;
use async_trait::async_trait;

use domain::entities::book::book_character::BookCharacter;
use domain::entities::book::book_involved::BookInvolved;
use domain::entities::genre::Genre;
use domain::entities::theme::Theme;
use domain::enums::language::Language;
use domain::items_total::ItemsTotal;
use domain::pagination::Pagination;

#[async_trait]
pub trait BookRelationsRepository: Send + Sync {
  async fn get_themes(&self, book_id: u32, language: Language,pagination: Pagination) -> Result<ItemsTotal<Theme>, Box<dyn Error>>;
  async fn get_genres(&self, book_id: u32, language: Language,pagination: Pagination) -> Result<ItemsTotal<Genre>, Box<dyn Error>>;
  async fn get_characters(&self, book_id: u32, language: Language, pagination: Pagination) -> Result<ItemsTotal<BookCharacter>, Box<dyn Error>>;
  async fn get_involved(&self, book_id: u32, language: Language, pagination: Pagination) -> Result<ItemsTotal<BookInvolved>, Box<dyn Error>>;
}
