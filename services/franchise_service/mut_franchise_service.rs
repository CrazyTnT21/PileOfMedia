use crate::join_comma::JoinComma;
use crate::traits::service_error::ServiceError;
use async_trait::async_trait;
use domain::entities::franchise::Franchise;
use domain::entities::franchise::create_franchise::CreateFranchise;
use std::error::Error;
use std::fmt::{Display, Formatter};

#[async_trait]
pub trait MutFranchiseService: Send + Sync {
  async fn create(&self, item: CreateFranchise) -> Result<Franchise, ServiceError<MutFranchiseServiceError>>;
  async fn delete(&self, ids: &[u32]) -> Result<(), ServiceError<MutFranchiseServiceError>>;
}

#[derive(Debug)]
pub enum MutFranchiseServiceError {
  NoIdsProvided,
  NonExistent(Vec<u32>),
  NoTranslationsProvided,
  InvalidName(String),
  OtherError(Box<dyn Error>),
}

impl Display for MutFranchiseServiceError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}",
      match self {
        MutFranchiseServiceError::OtherError(x) => x.to_string(),
        MutFranchiseServiceError::NoTranslationsProvided => "No translations provided".to_string(),
        MutFranchiseServiceError::InvalidName(x) => format!("Name '{x}' in translation is invalid"),
        MutFranchiseServiceError::NonExistent(x) =>
          format!("The following franchises do not exist: [{}]", x.join_comma()),
        MutFranchiseServiceError::NoIdsProvided => "No ids provided".to_string(),
      }
    )
  }
}

impl Error for MutFranchiseServiceError {
  fn source(&self) -> Option<&(dyn Error + 'static)> {
    match self {
      MutFranchiseServiceError::OtherError(error) => Some(&**error),
      _ => None,
    }
  }
}
