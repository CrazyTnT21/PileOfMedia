use std::sync::Arc;

use async_trait::async_trait;

use repositories::book_repository::book_character_repository::BookCharacterRepository;
use repositories::book_repository::book_character_repository::mut_book_character_repository::MutBookCharacterRepository;
use repositories::book_repository::BookRepository;
use repositories::character_repository::CharacterRepository;
use services::book_service::book_character_service::mut_book_character_service::{MutBookCharacterService, MutBookCharacterServiceError};
use services::traits::service_error::ServiceError;

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
  async fn add(&self, book_id: u32, characters: &[u32]) -> Result<(), ServiceError<MutBookCharacterServiceError>> {
    self.validate_add(book_id, characters).await?;
    Ok(self.mut_book_character_repository.add(book_id, characters).await?)
  }

  async fn remove(&self, book_id: u32, characters: &[u32]) -> Result<(), ServiceError<MutBookCharacterServiceError>> {
    self.validate_remove(book_id, characters).await?;
    Ok(self.mut_book_character_repository.remove(book_id, characters).await?)
  }
}

impl DefaultMutBookCharacterService<'_> {
  async fn validate_add(&self, book_id: u32, characters: &[u32]) -> Result<(), ServiceError<MutBookCharacterServiceError>> {
    self.validate(book_id, characters).await?;
    let associated = self.book_character_repository.filter_existing(book_id, characters).await?;
    if !associated.is_empty() {
      let error = MutBookCharacterServiceError::AlreadyAssociated(associated);
      return Err(ServiceError::ClientError(error));
    };
    let existing_characters = self.character_repository.filter_existing(characters).await?;
    if existing_characters.len() != characters.len() {
      let non_existent_characters = filter_non_existent(characters, &existing_characters);
      let error = MutBookCharacterServiceError::NonExistent(non_existent_characters);
      return Err(ServiceError::ClientError(error));
    };

    Ok(())
  }
  async fn validate_remove(&self, book_id: u32, characters: &[u32]) -> Result<(), ServiceError<MutBookCharacterServiceError>> {
    self.validate(book_id, characters).await?;
    let existing = self.book_character_repository.filter_existing(book_id, characters).await?;
    if existing.len() != characters.len() {
      let non_existent_characters = filter_non_existent(characters, &existing);
      return Err(ServiceError::ClientError(MutBookCharacterServiceError::NotAssociated(non_existent_characters)));
    };

    Ok(())
  }
  async fn validate(&self, book_id: u32, characters: &[u32]) -> Result<(), ServiceError<MutBookCharacterServiceError>> {
    let ids = self.book_repository.filter_existing(&[book_id]).await?;
    if ids.is_empty() {
      return Err(ServiceError::ClientError(MutBookCharacterServiceError::NonExistentBook(book_id)));
    }
    if characters.is_empty() {
      return Err(ServiceError::ClientError(MutBookCharacterServiceError::NoCharactersProvided));
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
