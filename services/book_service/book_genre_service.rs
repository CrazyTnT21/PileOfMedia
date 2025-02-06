use std::fmt::{Display, Formatter};

use async_trait::async_trait;

use domain::entities::genre::Genre;
use domain::enums::language::Language;

use crate::traits::service_error::ServiceError;

pub mod mut_book_genre_service;

#[async_trait]
pub trait BookGenreService: Send + Sync {
  async fn get_by_id(
    &self,
    book_id: u32,
    languages: &[Language],
  ) -> Result<Vec<Genre>, ServiceError<BookGenreServiceError>>;
}

pub enum BookGenreServiceError {}

impl Display for BookGenreServiceError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "")
  }
}
