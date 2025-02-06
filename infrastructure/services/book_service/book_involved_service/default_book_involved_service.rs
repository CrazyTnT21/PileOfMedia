use std::sync::Arc;

use async_trait::async_trait;

use domain::entities::book::book_involved::BookInvolved;
use domain::enums::language::Language;
use repositories::book_repository::book_involved_repository::BookInvolvedRepository;
use services::book_service::book_involved_service::{BookInvolvedService, BookInvolvedServiceError};
use services::traits::service_error::ServiceError;

pub struct DefaultBookInvolvedService<'a> {
  book_involved_repository: Arc<dyn BookInvolvedRepository + 'a>,
}

impl<'a> DefaultBookInvolvedService<'a> {
  pub fn new(book_involved_repository: Arc<dyn BookInvolvedRepository + 'a>) -> DefaultBookInvolvedService<'a> {
    DefaultBookInvolvedService {
      book_involved_repository,
    }
  }
}

#[async_trait]
impl BookInvolvedService for DefaultBookInvolvedService<'_> {
  async fn get_by_id(
    &self,
    book_id: u32,
    languages: &[Language],
  ) -> Result<Vec<BookInvolved>, ServiceError<BookInvolvedServiceError>> {
    Ok(self.book_involved_repository.get_by_id(book_id, languages).await?)
  }
}
