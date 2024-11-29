use std::sync::Arc;

use async_trait::async_trait;

use repositories::file_repository::FileRepository;
use services::file_service::{FileService, FileServiceError};
use services::traits::service_error::ServiceError;

pub struct DefaultFileService<'a> {
  repository: Arc<dyn FileRepository + 'a>,
}

impl<'a> DefaultFileService<'a> {
  pub fn new(repository: Arc<dyn FileRepository + 'a>) -> DefaultFileService<'a> {
    DefaultFileService { repository }
  }
}

#[async_trait]
impl FileService for DefaultFileService<'_> {
  async fn get(&self, uri: &str) -> Result<Vec<u8>, ServiceError<FileServiceError>> {
    Ok(self.repository.get(uri).await?)
  }
}
