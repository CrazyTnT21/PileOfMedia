use crate::join_comma::JoinComma;
use crate::traits::service_error::ServiceError;
use async_trait::async_trait;
use domain::entities::theme::create_theme::CreateTheme;
use domain::entities::theme::Theme;
use std::fmt::{Display, Formatter};

#[async_trait]
pub trait MutThemeService: Send + Sync {
  async fn create(&self, item: CreateTheme) -> Result<Theme, ServiceError<MutThemeServiceError>>;
  async fn delete(&self, ids: &[u32]) -> Result<(), ServiceError<MutThemeServiceError>>;
}

pub enum MutThemeServiceError {
  NoIdsProvided,
  NonExistent(Vec<u32>),
  NoTranslationsProvided,
  InvalidName(String),
  OtherError(Box<dyn Display>),
}

impl Display for MutThemeServiceError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}",
      match self {
        MutThemeServiceError::OtherError(x) => x.to_string(),
        MutThemeServiceError::NoTranslationsProvided => "No translations provided".to_string(),
        MutThemeServiceError::InvalidName(x) => format!("Name '{x}' in translation is invalid"),
        MutThemeServiceError::NonExistent(x) => format!("The following themes do not exist: [{}]", x.join_comma()),
        MutThemeServiceError::NoIdsProvided => "No ids provided".to_string(),
      }
    )
  }
}
