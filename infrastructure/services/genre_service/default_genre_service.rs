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
impl GenreService for DefaultGenreService<'_> {
  async fn get(
    &self,
    languages: &[Language],
    pagination: Pagination,
  ) -> Result<ItemsTotal<Genre>, ServiceError<GenreServiceError>> {
    Ok(self.genre_repository.get(languages, pagination).await?)
  }

  async fn get_by_id(&self, id: u32, languages: &[Language]) -> Result<Option<Genre>, ServiceError<GenreServiceError>> {
    Ok(self.genre_repository.get_by_id(id, languages).await?)
  }

  async fn get_by_name(
    &self,
    name: &str,
    languages: &[Language],
    pagination: Pagination,
  ) -> Result<ItemsTotal<Genre>, ServiceError<GenreServiceError>> {
    Ok(self.genre_repository.get_by_name(name, languages, pagination).await?)
  }
}
