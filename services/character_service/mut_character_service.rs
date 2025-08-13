use crate::join_comma::JoinComma;
use crate::traits::service_error::ServiceError;
use async_trait::async_trait;
use domain::entities::character::Character;
use domain::entities::character::create_character::CreateCharacter;
use domain::enums::language::Language;
use std::error::Error;
use std::fmt::{Display, Formatter};

#[async_trait]
pub trait MutCharacterService: Send + Sync {
  async fn create(&self, item: CreateCharacter) -> Result<Character, ServiceError<MutCharacterServiceError>>;
  async fn delete(&self, ids: &[u32]) -> Result<(), ServiceError<MutCharacterServiceError>>;
}

#[derive(Debug)]
pub enum MutCharacterServiceError {
  NoIdsProvided,
  NoTranslationsProvided,
  InvalidName(String),
  InvalidDescription(String),
  NonExistentTranslationImage(Language),
  NonExistentCharacters(Vec<u32>),
  OtherError(Box<dyn Error>),
}

impl Display for MutCharacterServiceError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}",
      match self {
        MutCharacterServiceError::OtherError(x) => x.to_string(),
        MutCharacterServiceError::NoTranslationsProvided => "No translations provided".to_string(),
        MutCharacterServiceError::InvalidName(x) => format!("Name '{x}' in translation is invalid"),
        MutCharacterServiceError::InvalidDescription(x) => format!("Description '{x}' in translation is invalid"),
        MutCharacterServiceError::NonExistentTranslationImage(language) => format!(
          "A cover for the language '{language}' ({}) does not exist",
          language.language_code()
        ),
        MutCharacterServiceError::NonExistentCharacters(x) =>
          format!("The following characters do not exist: [{}]", x.join_comma()),

        MutCharacterServiceError::NoIdsProvided => "No ids provided".to_string(),
      }
    )
  }
}

impl Error for MutCharacterServiceError {
  fn source(&self) -> Option<&(dyn Error + 'static)> {
    match self {
      MutCharacterServiceError::OtherError(error) => Some(&**error),
      _ => None,
    }
  }
}
