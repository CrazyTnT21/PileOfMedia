use std::error::Error;
use std::fmt::{Display, Formatter};

use async_trait::async_trait;

use crate::traits::service_error::ServiceError;
use domain::file_name::FileName;

#[async_trait]
pub trait MutFileService: Send + Sync {
  async fn create(
    &self,
    data: &[u8],
    file_path: &str,
    file_name: Option<&str>,
  ) -> Result<FileName, ServiceError<MutFileServiceError>>;
  async fn delete(&self, uri: &str) -> Result<(), ServiceError<MutFileServiceError>>;
}
#[derive(Debug)]
pub enum MutFileServiceError {
  OtherError(Box<dyn Error>),
}

impl Display for MutFileServiceError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}",
      match self {
        MutFileServiceError::OtherError(x) => x.to_string(),
      }
    )
  }
}
impl Error for MutFileServiceError {
  fn source(&self) -> Option<&(dyn Error + 'static)> {
    match self {
      MutFileServiceError::OtherError(error) => Some(&**error),
    }
  }
}
