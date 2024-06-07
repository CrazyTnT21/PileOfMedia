use async_trait::async_trait;
use crate::traits::service_error::ServiceError;

pub mod mut_file_service;

#[async_trait]
pub trait FileService: Send + Sync {
  async fn get(&self, uri: &str) -> Result<Vec<u8>, ServiceError>;
}
