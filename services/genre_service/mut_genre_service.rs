use crate::join_comma::JoinComma;
use crate::traits::service_error::ServiceError;
use async_trait::async_trait;
use domain::entities::genre::Genre;
use domain::entities::genre::create_genre::CreateGenre;
use std::fmt::{Display, Formatter};

#[async_trait]
pub trait MutGenreService: Send + Sync {
  async fn create(&self, item: CreateGenre) -> Result<Genre, ServiceError<MutGenreServiceError>>;
  async fn delete(&self, ids: &[u32]) -> Result<(), ServiceError<MutGenreServiceError>>;
}

pub enum MutGenreServiceError {
  NoIdsProvided,
  NonExistent(Vec<u32>),
  NoTranslationsProvided,
  InvalidName(String),
  OtherError(Box<dyn Display>),
}

impl Display for MutGenreServiceError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}",
      match self {
        MutGenreServiceError::OtherError(x) => x.to_string(),
        MutGenreServiceError::NoTranslationsProvided => "No translations provided".to_string(),
        MutGenreServiceError::InvalidName(x) => format!("Name '{x}' in translation is invalid"),
        MutGenreServiceError::NonExistent(x) => format!("The following genres do not exist: [{}]", x.join_comma()),
        MutGenreServiceError::NoIdsProvided => "No ids provided".to_string(),
      }
    )
  }
}
