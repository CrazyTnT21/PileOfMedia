use std::error::Error;
use std::fmt::{Display, Formatter};

use async_trait::async_trait;

use crate::traits::service_error::ServiceError;
use domain::entities::theme::Theme;
use domain::enums::language::Language;

pub mod mut_book_theme_service;

#[async_trait]
pub trait BookThemeService: Send + Sync {
  async fn get_by_id(
    &self,
    book_id: u32,
    languages: &[Language],
  ) -> Result<Vec<Theme>, ServiceError<BookThemeServiceError>>;
}
#[derive(Debug)]
pub enum BookThemeServiceError {
  OtherError(Box<dyn Error>),
}

impl Display for BookThemeServiceError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}",
      match self {
        BookThemeServiceError::OtherError(value) => value.to_string(),
      }
    )
  }
}

impl Error for BookThemeServiceError {
  fn source(&self) -> Option<&(dyn Error + 'static)> {
    match self {
      BookThemeServiceError::OtherError(error) => Some(&**error),
    }
  }
}
