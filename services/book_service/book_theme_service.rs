use std::fmt::{Display, Formatter};

use async_trait::async_trait;

use domain::entities::theme::Theme;
use domain::enums::language::Language;
use domain::items_total::ItemsTotal;
use domain::pagination::Pagination;

use crate::traits::service_error::ServiceError;

pub mod mut_book_theme_service;

#[async_trait]
pub trait BookThemeService: Send + Sync {
  async fn get(
    &self,
    book_id: u32,
    language: Language,
    pagination: Pagination,
  ) -> Result<ItemsTotal<Theme>, ServiceError<BookThemeServiceError>>;
}

pub enum BookThemeServiceError {}

impl Display for BookThemeServiceError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "")
  }
}
