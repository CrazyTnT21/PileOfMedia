use std::fmt::{Display, Formatter};

use async_trait::async_trait;

use domain::file_name::FileName;

use crate::traits::service_error::ServiceError;

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

pub enum MutFileServiceError {}

impl Display for MutFileServiceError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "")
  }
}
