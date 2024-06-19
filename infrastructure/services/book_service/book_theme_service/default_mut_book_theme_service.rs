use std::sync::Arc;

use async_trait::async_trait;

use repositories::book_repository::book_theme_repository::BookThemeRepository;
use repositories::book_repository::book_theme_repository::mut_book_theme_repository::MutBookThemeRepository;
use repositories::book_repository::BookRepository;
use repositories::theme_repository::ThemeRepository;
use services::book_service::book_theme_service::mut_book_theme_service::MutBookThemeService;
use services::traits::service_error::{ClientError, ServiceError};

use crate::services::map_server_error;

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
  async fn add(&self, book_id: u32, themes: &[u32]) -> Result<(), ServiceError> {
    self.validate_add(book_id, themes).await?;
    self.mut_book_theme_repository.add(book_id, themes).await.map_err(map_server_error)
  }

  async fn remove(&self, book_id: u32, themes: &[u32]) -> Result<(), ServiceError> {
    self.validate_remove(book_id, themes).await?;
    self.mut_book_theme_repository.remove(book_id, themes).await.map_err(map_server_error)
  }
}

impl DefaultMutBookThemeService<'_> {
  async fn validate_add(&self, book_id: u32, themes: &[u32]) -> Result<(), ServiceError> {
    self.validate(book_id, themes).await?;
    let existing = self.book_theme_repository.filter_existing(book_id, themes).await.map_err(map_server_error)?;
    if !existing.is_empty() {
      return Err(ServiceError::ClientError(ClientError {
        title: "Invalid themes".to_string(),
        description: Some(format!("The following themes already have an association: [{}]", existing.into_iter().map(|x| x.to_string()).collect::<Vec<String>>().join(","))),
      }));
    };
    let existing_themes = self.theme_repository.filter_existing(themes).await.map_err(map_server_error)?;
    if existing_themes.len() != themes.len() {
      let non_existent_themes: Vec<String> = filter_non_existent(themes, &existing_themes).into_iter().map(|x| x.to_string()).collect();
      return Err(ServiceError::ClientError(ClientError {
        title: "Invalid themes".to_string(),
        description: Some(format!("The following themes do not exist: [{}]", non_existent_themes.join(","))),
      }));
    };

    Ok(())
  }
  async fn validate_remove(&self, book_id: u32, themes: &[u32]) -> Result<(), ServiceError> {
    self.validate(book_id, themes).await?;
    let existing = self.book_theme_repository.filter_existing(book_id, themes).await.map_err(map_server_error)?;
    if existing.len() != themes.len() {
      let non_existent_themes: Vec<String> = filter_non_existent(themes, &existing).into_iter().map(|x| x.to_string()).collect();
      return Err(ServiceError::ClientError(ClientError {
        title: "Invalid themes".to_string(),
        description: Some(format!("The following themes do not have an association: [{}]", non_existent_themes.join(","))),
      }));
    };

    Ok(())
  }
  async fn validate(&self, book_id: u32, themes: &[u32]) -> Result<(), ServiceError> {
    let ids = self.book_repository.filter_existing(&[book_id]).await.map_err(map_server_error)?;
    if ids.is_empty() {
      return Err(ServiceError::ClientError(ClientError {
        title: format!("Book with the id {book_id} does not exist"),
        description: None,
      }));
    }
    if themes.is_empty() {
      return Err(ServiceError::ClientError(ClientError {
        title: "No themes provided".to_string(),
        description: None,
      }));
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
