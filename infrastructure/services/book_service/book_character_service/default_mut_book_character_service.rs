use std::sync::Arc;

use async_trait::async_trait;

use repositories::book_repository::book_character_repository::BookCharacterRepository;
use repositories::book_repository::book_character_repository::mut_book_character_repository::MutBookCharacterRepository;
use repositories::book_repository::BookRepository;
use repositories::character_repository::CharacterRepository;
use services::book_service::book_character_service::mut_book_character_service::MutBookCharacterService;
use services::traits::service_error::{ClientError, ServiceError};

use crate::services::map_server_error;

pub struct DefaultMutBookCharacterService<'a> {
  book_repository: Arc<dyn BookRepository + 'a>,
  book_character_repository: Arc<dyn BookCharacterRepository + 'a>,
  mut_book_character_repository: Arc<dyn MutBookCharacterRepository + 'a>,
  character_repository: Arc<dyn CharacterRepository + 'a>,
}

impl<'a> DefaultMutBookCharacterService<'a> {
  pub fn new(book_repository: Arc<dyn BookRepository + 'a>,
             book_character_repository: Arc<dyn BookCharacterRepository + 'a>,
             mut_book_character_repository: Arc<dyn MutBookCharacterRepository + 'a>,
             character_repository: Arc<dyn CharacterRepository + 'a>, ) -> DefaultMutBookCharacterService<'a> {
    DefaultMutBookCharacterService { book_repository, book_character_repository, mut_book_character_repository, character_repository }
  }
}

#[async_trait]
impl<'a> MutBookCharacterService for DefaultMutBookCharacterService<'a> {
  async fn add(&self, book_id: u32, characters: &[u32]) -> Result<(), ServiceError> {
    self.validate_add(book_id, characters).await?;
    self.mut_book_character_repository.add(book_id, characters).await.map_err(map_server_error)
  }

  async fn remove(&self, book_id: u32, characters: &[u32]) -> Result<(), ServiceError> {
    self.validate_remove(book_id, characters).await?;
    self.mut_book_character_repository.remove(book_id, characters).await.map_err(map_server_error)
  }
}

impl DefaultMutBookCharacterService<'_> {
  async fn validate_add(&self, book_id: u32, characters: &[u32]) -> Result<(), ServiceError> {
    self.validate(book_id, characters).await?;
    let existing = self.book_character_repository.filter_existing(book_id, characters).await.map_err(map_server_error)?;
    if !existing.is_empty() {
      return Err(ServiceError::ClientError(ClientError {
        title: "Invalid characters".to_string(),
        description: Some(format!("The following characters already have an association: [{}]", existing.into_iter().map(|x| x.to_string()).collect::<Vec<String>>().join(","))),
      }));
    };
    let existing_characters = self.character_repository.filter_existing(characters).await.map_err(map_server_error)?;
    if existing_characters.len() != characters.len() {
      let non_existent_characters: Vec<String> = filter_non_existent(characters, &existing_characters).into_iter().map(|x| x.to_string()).collect();
      return Err(ServiceError::ClientError(ClientError {
        title: "Invalid characters".to_string(),
        description: Some(format!("The following characters do not exist: [{}]", non_existent_characters.join(","))),
      }));
    };

    Ok(())
  }
  async fn validate_remove(&self, book_id: u32, characters: &[u32]) -> Result<(), ServiceError> {
    self.validate(book_id, characters).await?;
    let existing = self.book_character_repository.filter_existing(book_id, characters).await.map_err(map_server_error)?;
    if existing.len() != characters.len() {
      let non_existent_characters: Vec<String> = filter_non_existent(characters, &existing).into_iter().map(|x| x.to_string()).collect();
      return Err(ServiceError::ClientError(ClientError {
        title: "Invalid characters".to_string(),
        description: Some(format!("The following characters do not have an association: [{}]", non_existent_characters.join(","))),
      }));
    };

    Ok(())
  }
  async fn validate(&self, book_id: u32, characters: &[u32]) -> Result<(), ServiceError> {
    let ids = self.book_repository.filter_existing(&[book_id]).await.map_err(map_server_error)?;
    if ids.is_empty() {
      return Err(ServiceError::ClientError(ClientError {
        title: format!("Book with the id {book_id} does not exist"),
        description: None,
      }));
    }
    if characters.is_empty() {
      return Err(ServiceError::ClientError(ClientError {
        title: "No characters provided".to_string(),
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
