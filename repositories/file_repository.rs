use std::error::Error;
use async_trait::async_trait;

pub mod mut_file_repository;

#[async_trait]
pub trait FileRepository: Send + Sync {
  async fn get(&self, uri: &str) -> Result<Vec<u8>, Box<dyn Error>>;
}
