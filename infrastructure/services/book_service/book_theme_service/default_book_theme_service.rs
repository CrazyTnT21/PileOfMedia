use std::sync::Arc;

use async_trait::async_trait;

use domain::entities::theme::Theme;
use domain::enums::language::Language;
use domain::items_total::ItemsTotal;
use domain::pagination::Pagination;
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
  async fn get(
    &self,
    book_id: u32,
    language: Language,
    pagination: Pagination,
  ) -> Result<ItemsTotal<Theme>, ServiceError<BookThemeServiceError>> {
    Ok(self.book_theme_repository.get(book_id, language, pagination).await?)
  }
}
