pub mod mut_theme_service;

use std::error::Error;
use std::fmt::{Display, Formatter};

use async_trait::async_trait;

use domain::entities::theme::Theme;
use domain::enums::language::Language;
use domain::items_total::ItemsTotal;
use domain::pagination::Pagination;

use crate::traits::service_error::ServiceError;

#[async_trait]
pub trait ThemeService: Send + Sync {
  async fn get(
    &self,
    languages: &[Language],
    pagination: Pagination,
  ) -> Result<ItemsTotal<Theme>, ServiceError<ThemeServiceError>>;
  async fn get_by_id(&self, id: u32, languages: &[Language]) -> Result<Option<Theme>, ServiceError<ThemeServiceError>>;
  async fn get_by_name(
    &self,
    name: &str,
    languages: &[Language],
    pagination: Pagination,
  ) -> Result<ItemsTotal<Theme>, ServiceError<ThemeServiceError>>;
}

#[derive(Debug)]
pub enum ThemeServiceError {
  OtherError(Box<dyn Error>),
}

impl Display for ThemeServiceError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}",
      match self {
        ThemeServiceError::OtherError(value) => value.to_string(),
      }
    )
  }
}

impl Error for ThemeServiceError {
  fn source(&self) -> Option<&(dyn Error + 'static)> {
    match self {
      ThemeServiceError::OtherError(error) => Some(&**error),
    }
  }
}
