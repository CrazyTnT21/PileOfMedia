use std::sync::Arc;

use async_trait::async_trait;

use domain::entities::genre::Genre;
use domain::enums::language::Language;
use domain::items_total::ItemsTotal;
use domain::pagination::Pagination;
use repositories::genre_repository::GenreRepository;
use services::genre_service::{GenreService, GenreServiceError};
use services::traits::service_error::ServiceError;

pub struct DefaultGenreService<'a> {
  genre_repository: Arc<dyn GenreRepository + 'a>,
}

impl<'a> DefaultGenreService<'a> {
  pub fn new(genre_repository: Arc<dyn GenreRepository + 'a>) -> DefaultGenreService<'a> {
    DefaultGenreService { genre_repository }
  }
}

#[async_trait]
impl<'a> GenreService for DefaultGenreService<'a> {
  async fn get(
    &self,
    language: Language,
    pagination: Pagination,
  ) -> Result<ItemsTotal<Genre>, ServiceError<GenreServiceError>> {
    Ok(self.genre_repository.get(language, pagination).await?)
  }

  async fn get_by_id(&self, id: u32, language: Language) -> Result<Option<Genre>, ServiceError<GenreServiceError>> {
    Ok(self.genre_repository.get_by_id(id, language).await?)
  }

  async fn get_by_name(
    &self,
    name: &str,
    language: Language,
    pagination: Pagination,
  ) -> Result<ItemsTotal<Genre>, ServiceError<GenreServiceError>> {
    Ok(self.genre_repository.get_by_name(name, language, pagination).await?)
  }
}
