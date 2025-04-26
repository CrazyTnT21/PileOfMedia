use crate::join_comma::JoinComma;
use crate::traits::service_error::ServiceError;
use async_trait::async_trait;
use domain::entities::role::Role;
use domain::entities::role::create_role::CreateRole;
use domain::enums::language::Language;
use std::error::Error;
use std::fmt::{Display, Formatter};

#[async_trait]
pub trait MutRoleService: Send + Sync {
  async fn create(&self, item: CreateRole) -> Result<Role, ServiceError<MutRoleServiceError>>;
  async fn delete(&self, ids: &[u32]) -> Result<(), ServiceError<MutRoleServiceError>>;
}

#[derive(Debug)]
pub enum MutRoleServiceError {
  NoIdsProvided,
  NonExistent(Vec<u32>),
  NoTranslationsProvided,
  NoTranslationInLanguageProvided(Language),
  InvalidName(String),
  OtherError(Box<dyn Error>),
}

impl Display for MutRoleServiceError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}",
      match self {
        MutRoleServiceError::OtherError(x) => x.to_string(),
        MutRoleServiceError::NoTranslationsProvided => "No translations provided".to_string(),
        MutRoleServiceError::NoTranslationInLanguageProvided(language) => format!(
          "No translation in '{}' ({}) provided",
          language,
          language.language_code()
        ),
        MutRoleServiceError::InvalidName(x) => format!("Name '{x}' in translation is invalid"),
        MutRoleServiceError::NonExistent(x) => format!("The following roles do not exist: [{}]", x.join_comma()),
        MutRoleServiceError::NoIdsProvided => "No ids provided".to_string(),
      }
    )
  }
}

impl Error for MutRoleServiceError {
  fn source(&self) -> Option<&(dyn Error + 'static)> {
    match self {
      MutRoleServiceError::OtherError(error) => Some(&**error),
      _ => None,
    }
  }
}
