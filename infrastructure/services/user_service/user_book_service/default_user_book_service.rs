use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;
use domain::entities::user::user_book::UserBook;
use domain::enums::language::Language;
use repositories::user_repository::user_book_repository::UserBookRepository;
use services::traits::service_error::ServiceError;
use services::user_service::user_book_service::{UserBookService, UserBookServiceError};

pub struct DefaultUserBookService<'a> {
  user_book_repository: Arc<dyn UserBookRepository + 'a>,
}

impl<'a> DefaultUserBookService<'a> {
  pub fn new(user_book_repository: Arc<dyn UserBookRepository + 'a>) -> DefaultUserBookService<'a> {
    DefaultUserBookService { user_book_repository }
  }
}

#[async_trait]
impl UserBookService for DefaultUserBookService<'_> {
  async fn get_by_book_id(
    &self,
    user_id: u32,
    book_id: u32,
    languages: &[Language],
  ) -> Result<Option<UserBook>, ServiceError<UserBookServiceError>> {
    Ok(
      self
        .user_book_repository
        .get_by_book_id(user_id, book_id, languages)
        .await?,
    )
  }

  async fn get_by_book_ids(
    &self,
    user_id: u32,
    book_ids: &[u32],
    languages: &[Language],
  ) -> Result<Vec<UserBook>, ServiceError<UserBookServiceError>> {
    Ok(
      self
        .user_book_repository
        .get_by_book_ids(user_id, book_ids, languages)
        .await?,
    )
  }

  async fn get_by_user_id(
    &self,
    user_id: u32,
    languages: &[Language],
  ) -> Result<Vec<UserBook>, ServiceError<UserBookServiceError>> {
    Ok(self.user_book_repository.get_by_user_id(user_id, languages).await?)
  }

  async fn get_by_user_ids(
    &self,
    user_ids: &[u32],
    languages: &[Language],
  ) -> Result<HashMap<u32, Vec<UserBook>>, ServiceError<UserBookServiceError>> {
    Ok(self.user_book_repository.get_by_user_ids(user_ids, languages).await?)
  }
}
