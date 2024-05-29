use std::error::Error;
use async_trait::async_trait;

#[async_trait]
pub trait FileRepository: Send + Sync {
  async fn create(&self, data: &[u8], file_path: &str, file_name: Option<&str>) -> Result<String, Box<dyn Error>>;
  async fn create_base64(&self, data: &str, file_path: &str, file_name: Option<&str>) -> Result<String, Box<dyn Error>>;
}
