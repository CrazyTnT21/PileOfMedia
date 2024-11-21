use async_trait::async_trait;
use std::error::Error;
use std::fs;

use repositories::file_repository::FileRepository;

pub struct DefaultFileRepository {}

impl DefaultFileRepository {
  pub const fn new() -> DefaultFileRepository {
    DefaultFileRepository {}
  }
}
impl Default for DefaultFileRepository {
  fn default() -> Self {
    Self::new()
  }
}
#[async_trait]
impl FileRepository for DefaultFileRepository {
  async fn get(&self, uri: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    Ok(fs::read(uri)?)
  }
}
