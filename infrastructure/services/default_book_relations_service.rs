use std::sync::Arc;
use async_trait::async_trait;

use domain::entities::book::book_character::BookCharacter;
use domain::entities::book::book_involved::BookInvolved;
use domain::entities::genre::Genre;
use domain::entities::theme::Theme;
use domain::enums::language::Language;
use domain::items_total::ItemsTotal;
use domain::pagination::Pagination;
use repositories::book_relations_repository::BookRelationsRepository;
use services::book_relations_service::BookRelationsService;
use services::traits::service_error::ServiceError;

use crate::services::map_server_error;

pub struct DefaultBookRelationsService<'a> {
  book_relations_repository: Arc<dyn BookRelationsRepository + 'a>,
}

impl<'a> DefaultBookRelationsService<'a> {
  pub fn new(book_relations_repository: Arc<dyn BookRelationsRepository + 'a>) -> DefaultBookRelationsService<'a> {
    DefaultBookRelationsService { book_relations_repository }
  }
}

#[async_trait]
impl<'a> BookRelationsService for DefaultBookRelationsService<'a> {
  async fn get_themes(&self, book_id: u32, language: Language, pagination: Pagination) -> Result<ItemsTotal<Theme>, ServiceError> {
    self.book_relations_repository.get_themes(book_id, language, pagination).await.map_err(map_server_error)
  }

  async fn get_genres(&self, book_id: u32, language: Language, pagination: Pagination) -> Result<ItemsTotal<Genre>, ServiceError> {
    self.book_relations_repository.get_genres(book_id, language, pagination).await.map_err(map_server_error)
  }

  async fn get_characters(&self, book_id: u32, language: Language, pagination: Pagination) -> Result<ItemsTotal<BookCharacter>, ServiceError> {
    self.book_relations_repository.get_characters(book_id, language, pagination).await.map_err(map_server_error)
  }

  async fn get_involved(&self, book_id: u32, language: Language, pagination: Pagination) -> Result<ItemsTotal<BookInvolved>, ServiceError> {
    self.book_relations_repository.get_involved(book_id, language, pagination).await.map_err(map_server_error)
  }
}
