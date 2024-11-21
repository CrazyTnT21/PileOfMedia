use std::sync::Arc;

use async_trait::async_trait;

use domain::entities::book::book_involved::BookInvolved;
use domain::enums::language::Language;
use domain::items_total::ItemsTotal;
use domain::pagination::Pagination;
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
impl<'a> BookInvolvedService for DefaultBookInvolvedService<'a> {
  async fn get(
    &self,
    book_id: u32,
    language: Language,
    pagination: Pagination,
  ) -> Result<ItemsTotal<BookInvolved>, ServiceError<BookInvolvedServiceError>> {
    Ok(self.book_involved_repository.get(book_id, language, pagination).await?)
  }
}
