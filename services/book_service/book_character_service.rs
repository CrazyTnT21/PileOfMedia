use std::fmt::{Display, Formatter};

use async_trait::async_trait;
use domain::entities::book::book_character::BookCharacter;
use domain::enums::language::Language;

use crate::traits::service_error::ServiceError;

pub mod mut_book_character_service;

#[async_trait]
pub trait BookCharacterService: Send + Sync {
  async fn get_by_id(
    &self,
    book_id: u32,
    languages: &[Language],
  ) -> Result<Vec<BookCharacter>, ServiceError<BookCharacterServiceError>>;
}

pub enum BookCharacterServiceError {}

impl Display for BookCharacterServiceError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "")
  }
}
