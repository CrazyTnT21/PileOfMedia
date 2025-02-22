use crate::join_comma::JoinComma;
use crate::traits::service_error::ServiceError;
use async_trait::async_trait;
use domain::entities::book::Book;
use domain::entities::book::create_book::CreateBook;
use domain::enums::language::Language;
use std::fmt::{Display, Formatter};

#[async_trait]
pub trait MutBookService: Send + Sync {
  async fn create(&self, item: CreateBook) -> Result<Book, ServiceError<MutBookServiceError>>;
  async fn delete(&self, ids: &[u32]) -> Result<(), ServiceError<MutBookServiceError>>;
}

pub enum MutBookServiceError {
  NonExistentFranchise(u32),
  NoIdsProvided,
  NonExistentBooks(Vec<u32>),
  NoTranslationsProvided,
  NonExistentThemes(Vec<u32>),
  NonExistentGenres(Vec<u32>),
  NonExistentPeople(Vec<u32>),
  NonExistentRoles(Vec<u32>),
  NonExistentCharacters(Vec<u32>),
  InvalidTitle(String),
  InvalidDescription(String),
  AlreadyExistingSlug(String),
  NonExistentTranslationCover(Language),
  OtherError(Box<dyn Display>),
}

impl Display for MutBookServiceError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}",
      match self {
        MutBookServiceError::NonExistentFranchise(x) => format!("Franchise with the id '{x}' does not exist"),
        MutBookServiceError::OtherError(x) => x.to_string(),
        MutBookServiceError::NonExistentThemes(x) => format!("The following themes do not exist: [{}]", x.join_comma()),
        MutBookServiceError::NonExistentGenres(x) => format!("The following genres do not exist: [{}]", x.join_comma()),
        MutBookServiceError::NonExistentCharacters(x) =>
          format!("The following characters do not exist: [{}]", x.join_comma()),
        MutBookServiceError::NonExistentPeople(x) => format!("The following people do not exist: [{}]", x.join_comma()),
        MutBookServiceError::NonExistentRoles(x) => format!("The following roles do not exist: [{}]", x.join_comma()),
        MutBookServiceError::NoTranslationsProvided => "No translations provided".to_string(),
        MutBookServiceError::InvalidTitle(x) => format!("Title '{x}' in translation is invalid"),
        MutBookServiceError::InvalidDescription(x) => format!("Description '{x}' in translation is invalid"),
        MutBookServiceError::NonExistentTranslationCover(language) => format!(
          "A cover for the language '{language}' ({}) does not exist",
          language.language_code()
        ),
        MutBookServiceError::NonExistentBooks(x) =>
          format!("Books with the following ids do not exist: [{}]", x.join_comma()),
        MutBookServiceError::NoIdsProvided => "No ids provided".to_string(),
        MutBookServiceError::AlreadyExistingSlug(slug) =>
          format!("A book with the following slug already exists: {slug}"),
      }
    )
  }
}
