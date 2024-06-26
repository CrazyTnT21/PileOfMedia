use std::sync::Arc;

use async_trait::async_trait;

use repositories::book_repository::book_genre_repository::BookGenreRepository;
use repositories::book_repository::book_genre_repository::mut_book_genre_repository::MutBookGenreRepository;
use repositories::book_repository::BookRepository;
use repositories::genre_repository::GenreRepository;
use services::book_service::book_genre_service::mut_book_genre_service::{MutBookGenreService, MutBookGenreServiceError};
use services::traits::service_error::ServiceError;

use crate::services::map_server_error;

pub struct DefaultMutBookGenreService<'a> {
  book_repository: Arc<dyn BookRepository + 'a>,
  book_genre_repository: Arc<dyn BookGenreRepository + 'a>,
  mut_book_genre_repository: Arc<dyn MutBookGenreRepository + 'a>,
  genre_repository: Arc<dyn GenreRepository + 'a>,
}

impl<'a> DefaultMutBookGenreService<'a> {
  pub fn new(book_repository: Arc<dyn BookRepository + 'a>,
             book_genre_repository: Arc<dyn BookGenreRepository + 'a>,
             mut_book_genre_repository: Arc<dyn MutBookGenreRepository + 'a>,
             genre_repository: Arc<dyn GenreRepository + 'a>, ) -> DefaultMutBookGenreService<'a> {
    DefaultMutBookGenreService { book_repository, book_genre_repository, mut_book_genre_repository, genre_repository }
  }
}

#[async_trait]
impl<'a> MutBookGenreService for DefaultMutBookGenreService<'a> {
  async fn add(&self, book_id: u32, genres: &[u32]) -> Result<(), ServiceError<MutBookGenreServiceError>> {
    self.validate_add(book_id, genres).await?;
    self.mut_book_genre_repository.add(book_id, genres).await.map_err(map_server_error)
  }

  async fn remove(&self, book_id: u32, genres: &[u32]) -> Result<(), ServiceError<MutBookGenreServiceError>> {
    self.validate_remove(book_id, genres).await?;
    self.mut_book_genre_repository.remove(book_id, genres).await.map_err(map_server_error)
  }
}

impl DefaultMutBookGenreService<'_> {
  async fn validate_add(&self, book_id: u32, genres: &[u32]) -> Result<(), ServiceError<MutBookGenreServiceError>> {
    self.validate(book_id, genres).await?;
    let existing = self.book_genre_repository.filter_existing(book_id, genres).await.map_err(map_server_error)?;
    if !existing.is_empty() {
      return Err(ServiceError::ClientError(MutBookGenreServiceError::AlreadyAssociated(existing)));
    };
    let existing_genres = self.genre_repository.filter_existing(genres).await.map_err(map_server_error)?;
    if existing_genres.len() != genres.len() {
      let non_existent_genres = filter_non_existent(genres, &existing_genres);
      return Err(ServiceError::ClientError(MutBookGenreServiceError::NonExistent(non_existent_genres)));
    };

    Ok(())
  }
  async fn validate_remove(&self, book_id: u32, genres: &[u32]) -> Result<(), ServiceError<MutBookGenreServiceError>> {
    self.validate(book_id, genres).await?;
    let existing = self.book_genre_repository.filter_existing(book_id, genres).await.map_err(map_server_error)?;
    if existing.len() != genres.len() {
      let not_associated = filter_non_existent(genres, &existing);
      return Err(ServiceError::ClientError(MutBookGenreServiceError::NotAssociated(not_associated)));
    };

    Ok(())
  }
  async fn validate(&self, book_id: u32, genres: &[u32]) -> Result<(), ServiceError<MutBookGenreServiceError>> {
    let ids = self.book_repository.filter_existing(&[book_id]).await.map_err(map_server_error)?;
    if ids.is_empty() {
      return Err(ServiceError::ClientError(MutBookGenreServiceError::NonExistentBook(book_id)));
    }
    if genres.is_empty() {
      return Err(ServiceError::ClientError(MutBookGenreServiceError::NoGenresProvided));
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
