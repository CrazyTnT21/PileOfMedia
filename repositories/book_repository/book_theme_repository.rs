pub mod mut_book_theme_repository;

use std::error::Error;

use async_trait::async_trait;

use domain::entities::theme::Theme;
use domain::enums::language::Language;

#[async_trait]
pub trait BookThemeRepository: Send + Sync {
  async fn get(&self, book_id: u32, languages: &[Language]) -> Result<Vec<Theme>, Box<dyn Error>>;
  async fn filter_existing(&self, book_id: u32, themes: &[u32]) -> Result<Vec<u32>, Box<dyn Error>>;
}
