use std::sync::Arc;

use async_trait::async_trait;

use repositories::book_repository::book_theme_repository::BookThemeRepository;
use repositories::book_repository::book_theme_repository::mut_book_theme_repository::MutBookThemeRepository;
use repositories::book_repository::BookRepository;
use repositories::theme_repository::ThemeRepository;
use services::book_service::book_theme_service::mut_book_theme_service::{MutBookThemeService, MutBookThemeServiceError};
use services::traits::service_error::ServiceError;

pub struct DefaultMutBookThemeService<'a> {
  book_repository: Arc<dyn BookRepository + 'a>,
  book_theme_repository: Arc<dyn BookThemeRepository + 'a>,
  mut_book_theme_repository: Arc<dyn MutBookThemeRepository + 'a>,
  theme_repository: Arc<dyn ThemeRepository + 'a>,
}

impl<'a> DefaultMutBookThemeService<'a> {
  pub fn new(book_repository: Arc<dyn BookRepository + 'a>,
             book_theme_repository: Arc<dyn BookThemeRepository + 'a>,
             mut_book_theme_repository: Arc<dyn MutBookThemeRepository + 'a>,
             theme_repository: Arc<dyn ThemeRepository + 'a>, ) -> DefaultMutBookThemeService<'a> {
    DefaultMutBookThemeService { book_repository, book_theme_repository, mut_book_theme_repository, theme_repository }
  }
}

#[async_trait]
impl<'a> MutBookThemeService for DefaultMutBookThemeService<'a> {
  async fn add(&self, book_id: u32, themes: &[u32]) -> Result<(), ServiceError<MutBookThemeServiceError>> {
    self.validate_add(book_id, themes).await?;
    Ok(self.mut_book_theme_repository.add(book_id, themes).await?)
  }

  async fn remove(&self, book_id: u32, themes: &[u32]) -> Result<(), ServiceError<MutBookThemeServiceError>> {
    self.validate_remove(book_id, themes).await?;
    Ok(self.mut_book_theme_repository.remove(book_id, themes).await?)
  }
}

impl DefaultMutBookThemeService<'_> {
  async fn validate_add(&self, book_id: u32, themes: &[u32]) -> Result<(), ServiceError<MutBookThemeServiceError>> {
    self.validate(book_id, themes).await?;
    let existing = self.book_theme_repository.filter_existing(book_id, themes).await?;
    if !existing.is_empty() {
      return Err(ServiceError::ClientError(MutBookThemeServiceError::AlreadyAssociated(existing)));
    };
    let existing_themes = self.theme_repository.filter_existing(themes).await?;
    if existing_themes.len() != themes.len() {
      let non_existent_themes = filter_non_existent(themes, &existing_themes);
      return Err(ServiceError::ClientError(MutBookThemeServiceError::NonExistent(non_existent_themes)));
    };

    Ok(())
  }
  async fn validate_remove(&self, book_id: u32, themes: &[u32]) -> Result<(), ServiceError<MutBookThemeServiceError>> {
    self.validate(book_id, themes).await?;
    let existing = self.book_theme_repository.filter_existing(book_id, themes).await?;
    if existing.len() != themes.len() {
      let not_associated = filter_non_existent(themes, &existing);
      return Err(ServiceError::ClientError(MutBookThemeServiceError::NotAssociated(not_associated)));
    };

    Ok(())
  }
  async fn validate(&self, book_id: u32, themes: &[u32]) -> Result<(), ServiceError<MutBookThemeServiceError>> {
    let ids = self.book_repository.filter_existing(&[book_id]).await?;
    if ids.is_empty() {
      return Err(ServiceError::ClientError(MutBookThemeServiceError::NonExistentBook(book_id)));
    }
    if themes.is_empty() {
      return Err(ServiceError::ClientError(MutBookThemeServiceError::NoThemesProvided));
    }
    Ok(())
  }
}

fn filter_non_existent(items: &[u32], existing: &[u32]) -> Vec<u32> {
  items.iter().filter_map(|x|
    existing.iter()
      .find(|y| **y == *x)
      .map(|_| None)
      .unwrap_or(Some(*x))
  ).collect()
}
