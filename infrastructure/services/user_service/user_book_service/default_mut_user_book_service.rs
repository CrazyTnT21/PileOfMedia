use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;
use domain::entities::user::create_user_book::CreateUserBook;
use domain::entities::user::user_book::UserBook;
use domain::enums::language::Language;
use repositories::book_repository::BookRepository;
use repositories::user_repository::UserRepository;
use repositories::user_repository::user_book_repository::UserBookRepository;
use repositories::user_repository::user_book_repository::mut_user_book_repository::MutUserBookRepository;
use services::traits::service_error::ServiceError;
use services::user_service::user_book_service::mut_user_book_service::{MutUserBookService, MutUserBookServiceError};

pub struct DefaultMutUserBookService<'a> {
  user_repository: Arc<dyn UserRepository + 'a>,
  user_book_repository: Arc<dyn UserBookRepository + 'a>,
  mut_user_book_repository: Arc<dyn MutUserBookRepository + 'a>,
  book_repository: Arc<dyn BookRepository + 'a>,
}

impl<'a> DefaultMutUserBookService<'a> {
  pub fn new(
    user_repository: Arc<dyn UserRepository + 'a>,
    user_book_repository: Arc<dyn UserBookRepository + 'a>,
    mut_user_book_repository: Arc<dyn MutUserBookRepository + 'a>,
    book_repository: Arc<dyn BookRepository + 'a>,
  ) -> DefaultMutUserBookService<'a> {
    DefaultMutUserBookService {
      user_repository,
      user_book_repository,
      mut_user_book_repository,
      book_repository,
    }
  }
}

#[async_trait]
impl MutUserBookService for DefaultMutUserBookService<'_> {
  async fn add(
    &self,
    user_id: u32,
    book: CreateUserBook,
    languages: &[Language],
  ) -> Result<UserBook, ServiceError<MutUserBookServiceError>> {
    self.validate_add(user_id, &book).await?;
    Ok(self.mut_user_book_repository.add(user_id, book, languages).await?)
  }

  async fn remove(&self, user_id: u32, book_ids: &[u32]) -> Result<(), ServiceError<MutUserBookServiceError>> {
    self.validate_remove(user_id, book_ids).await?;
    Ok(self.mut_user_book_repository.remove(user_id, book_ids).await?)
  }
}

impl DefaultMutUserBookService<'_> {
  async fn validate_add(
    &self,
    user_id: u32,
    item: &CreateUserBook,
  ) -> Result<(), ServiceError<MutUserBookServiceError>> {
    self.validate(user_id).await?;
    let mut map = HashMap::new();
    map.insert(user_id, vec![item.book_id]);
    let associated = self.user_book_repository.filter_existing(map).await?;
    if !associated.is_empty() {
      let error = MutUserBookServiceError::AlreadyAssociated(associated);
      return Err(ServiceError::ClientError(error));
    };
    let existing_books = self.book_repository.filter_existing(&[item.book_id]).await?;
    if existing_books.is_empty() {
      let error = MutUserBookServiceError::NonExistent(vec![item.book_id]);
      return Err(ServiceError::ClientError(error));
    };

    Ok(())
  }
  async fn validate_remove(&self, user_id: u32, book_ids: &[u32]) -> Result<(), ServiceError<MutUserBookServiceError>> {
    self.validate(user_id).await?;
    let mut map = HashMap::new();
    map.insert(user_id, book_ids.to_vec());
    let existing = self.user_book_repository.filter_existing(map).await?;
    if existing.len() != book_ids.len() {
      let non_existent_books =
        filter_non_existent(book_ids, &existing.values().flatten().copied().collect::<Vec<u32>>());
      return Err(ServiceError::ClientError(MutUserBookServiceError::NotAssociated(
        non_existent_books,
      )));
    };
    Ok(())
  }

  async fn validate(&self, user_id: u32) -> Result<(), ServiceError<MutUserBookServiceError>> {
    let ids = self.user_repository.filter_existing(&[user_id]).await?;
    if ids.is_empty() {
      return Err(ServiceError::ClientError(MutUserBookServiceError::NonExistentUser(
        user_id,
      )));
    }
    Ok(())
  }
}

fn filter_non_existent(items: &[u32], existing: &[u32]) -> Vec<u32> {
  items
    .iter()
    .filter_map(|x| existing.iter().find(|y| **y == *x).map_or(Some(*x), |_| None))
    .collect()
}
