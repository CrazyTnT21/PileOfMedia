use std::sync::Arc;

use async_trait::async_trait;

use domain::entities::genre::Genre;
use domain::enums::language::Language;
use domain::items_total::ItemsTotal;
use domain::pagination::Pagination;
use repositories::book_repository::book_genre_repository::BookGenreRepository;
use services::book_service::book_genre_service::BookGenreService;
use services::traits::service_error::ServiceError;

use crate::services::map_server_error;

pub struct DefaultBookGenreService<'a> {
  book_genre_repository: Arc<dyn BookGenreRepository + 'a>,
}

impl<'a> DefaultBookGenreService<'a> {
  pub fn new(book_genre_repository: Arc<dyn BookGenreRepository + 'a>) -> DefaultBookGenreService<'a> {
    DefaultBookGenreService { book_genre_repository }
  }
}

#[async_trait]
impl<'a> BookGenreService for DefaultBookGenreService<'a> {
  async fn get(&self, book_id: u32, language: Language, pagination: Pagination) -> Result<ItemsTotal<Genre>, ServiceError> {
    self.book_genre_repository.get(book_id, language, pagination).await.map_err(map_server_error)
  }
}
