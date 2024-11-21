use async_trait::async_trait;
use domain::file_name::FileName;
use std::error::Error;

#[async_trait]
pub trait MutFileRepository: Send + Sync {
  async fn create(&self, data: &[u8], file_path: &str, file_name: Option<&str>) -> Result<FileName, Box<dyn Error>>;
  async fn delete(&self, uri: &str) -> Result<(), Box<dyn Error>>;
}
