use std::sync::Arc;

use async_trait::async_trait;

use domain::entities::theme::Theme;
use domain::enums::language::Language;
use repositories::book_repository::book_theme_repository::BookThemeRepository;
use services::book_service::book_theme_service::{BookThemeService, BookThemeServiceError};
use services::traits::service_error::ServiceError;

pub struct DefaultBookThemeService<'a> {
  book_theme_repository: Arc<dyn BookThemeRepository + 'a>,
}

impl<'a> DefaultBookThemeService<'a> {
  pub fn new(book_theme_repository: Arc<dyn BookThemeRepository + 'a>) -> DefaultBookThemeService<'a> {
    DefaultBookThemeService { book_theme_repository }
  }
}

#[async_trait]
impl BookThemeService for DefaultBookThemeService<'_> {
  async fn get(&self, book_id: u32, languages: &[Language]) -> Result<Vec<Theme>, ServiceError<BookThemeServiceError>> {
    Ok(self.book_theme_repository.get(book_id, languages).await?)
  }
}
