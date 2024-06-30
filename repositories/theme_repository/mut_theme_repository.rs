use std::error::Error;
use async_trait::async_trait;
use domain::entities::theme::Theme;
use domain::entities::theme::create_partial_theme::CreatePartialTheme;

#[async_trait]
pub trait MutThemeRepository: Send + Sync {
  async fn create(&self, item: CreatePartialTheme) -> Result<Theme, Box<dyn Error>>;
  async fn delete(&self, ids: &[u32]) -> Result<(), Box<dyn Error>>;
}
