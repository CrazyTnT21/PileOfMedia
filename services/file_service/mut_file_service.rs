use async_trait::async_trait;
use domain::file_name::FileName;

use crate::traits::service_error::ServiceError;

#[async_trait]
pub trait MutFileService: Send + Sync {
  async fn create(&self, data: &[u8], file_path: &str, file_name: Option<&str>) -> Result<FileName, ServiceError>;
  async fn create_base64(&self, data: &str, file_path: &str, file_name: Option<&str>) -> Result<FileName, ServiceError>;
  async fn delete(&self, uri: &str) -> Result<(), ServiceError>;
}
