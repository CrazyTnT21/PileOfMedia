use std::fmt::{Display, Formatter};

use async_trait::async_trait;

use domain::entities::person::create_person::CreatePerson;
use domain::entities::person::Person;
use domain::enums::language::Language;

use crate::join_comma::JoinComma;
use crate::traits::service_error::ServiceError;

#[async_trait]
pub trait MutPersonService: Send + Sync {
  async fn create(&self, item: CreatePerson) -> Result<Person, ServiceError<MutPersonServiceError>>;
  async fn delete(&self, ids: &[u32]) -> Result<(), ServiceError<MutPersonServiceError>>;
}

pub enum MutPersonServiceError {
  InvalidName(String),
  InvalidDescription(String),
  OtherError(Box<dyn Display>),
  NoIdsProvided,
  NonExistentPeople(Vec<u32>),
  NoTranslationsProvided,
  NoTranslationInLanguageProvided(Language),
}

impl Display for MutPersonServiceError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", match self {
      MutPersonServiceError::OtherError(x) => x.to_string(),
      MutPersonServiceError::InvalidName(x) => format!("Name '{x}' is invalid"),
      MutPersonServiceError::InvalidDescription(x) => format!("Description '{x}' in translation is invalid"),
      MutPersonServiceError::NoIdsProvided => "No ids provided".to_string(),
      MutPersonServiceError::NonExistentPeople(x) => format!("The following people do not exist: [{}]", x.join_comma()),
      MutPersonServiceError::NoTranslationsProvided => "No translations provided".to_string(),
      MutPersonServiceError::NoTranslationInLanguageProvided(language) => format!("No translation in '{}' ({}) provided", language, language.language_code()),
    })
  }
}
