use std::sync::Arc;

use async_trait::async_trait;

use domain::entities::theme::Theme;
use domain::enums::language::Language;
use domain::items_total::ItemsTotal;
use domain::pagination::Pagination;
use repositories::theme_repository::ThemeRepository;
use services::theme_service::{ThemeService, ThemeServiceError};
use services::traits::service_error::ServiceError;

use crate::services::map_server_error;

pub struct DefaultThemeService<'a> {
  theme_repository: Arc<dyn ThemeRepository + 'a>,
}

impl<'a> DefaultThemeService<'a> {
  pub fn new(theme_repository: Arc<dyn ThemeRepository + 'a>) -> DefaultThemeService {
    DefaultThemeService { theme_repository }
  }
}

#[async_trait]
impl<'a> ThemeService for DefaultThemeService<'a> {
  async fn get(&self, language: Language, pagination: Pagination) -> Result<ItemsTotal<Theme>, ServiceError<ThemeServiceError>> {
    self.theme_repository.get(language, pagination).await.map_err(map_server_error)
  }

  async fn get_by_id(&self, id: u32, language: Language) -> Result<Option<Theme>, ServiceError<ThemeServiceError>> {
    self.theme_repository.get_by_id(id, language).await.map_err(map_server_error)
  }

  async fn get_by_name(&self, name: &str, language: Language, pagination: Pagination) -> Result<ItemsTotal<Theme>, ServiceError<ThemeServiceError>> {
    self.theme_repository.get_by_name(name, language, pagination).await.map_err(map_server_error)
  }
}
