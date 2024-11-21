use std::sync::Arc;

use async_trait::async_trait;

use domain::entities::book::Book;
use domain::enums::language::Language;
use domain::items_total::ItemsTotal;
use domain::pagination::Pagination;
use repositories::book_repository::BookRepository;
use services::book_service::{BookService, BookServiceError};
use services::traits::service_error::ServiceError;

pub struct DefaultBookService<'a> {
  book_repository: Arc<dyn BookRepository + 'a>,
}

impl<'a> DefaultBookService<'a> {
  pub fn new(book_repository: Arc<dyn BookRepository + 'a>) -> DefaultBookService {
    DefaultBookService { book_repository }
  }
}

#[async_trait]
impl<'a> BookService for DefaultBookService<'a> {
  async fn get(
    &self,
    language: Language,
    pagination: Pagination,
  ) -> Result<ItemsTotal<Book>, ServiceError<BookServiceError>> {
    Ok(self.book_repository.get(language, pagination).await?)
  }

  async fn get_by_id(&self, id: u32, language: Language) -> Result<Option<Book>, ServiceError<BookServiceError>> {
    Ok(self.book_repository.get_by_id(id, language).await?)
  }

  async fn get_by_title(
    &self,
    title: &str,
    language: Language,
    pagination: Pagination,
  ) -> Result<ItemsTotal<Book>, ServiceError<BookServiceError>> {
    Ok(self.book_repository.get_by_title(title, language, pagination).await?)
  }
}
