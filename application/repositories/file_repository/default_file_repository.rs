use std::error::Error;
use std::fs;
use async_trait::async_trait;

use repositories::file_repository::FileRepository;

pub struct DefaultFileRepository {}

impl DefaultFileRepository {
  pub fn new() -> DefaultFileRepository {
    DefaultFileRepository {}
  }
}

#[async_trait]
impl FileRepository for DefaultFileRepository {
  async fn get(&self, uri: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    Ok(fs::read(uri)?)
  }
}
