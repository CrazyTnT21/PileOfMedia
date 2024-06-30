use std::sync::Arc;

use async_trait::async_trait;

use domain::file_name::FileName;
use repositories::file_repository::mut_file_repository::MutFileRepository;
use services::file_service::mut_file_service::{MutFileService, MutFileServiceError};
use services::traits::service_error::ServiceError;

pub struct DefaultMutFileService<'a> {
  repository: Arc<dyn MutFileRepository + 'a>,
}

impl<'a> DefaultMutFileService<'a> {
  pub fn new(repository: Arc<dyn MutFileRepository + 'a>) -> DefaultMutFileService<'a> {
    DefaultMutFileService { repository }
  }
}

#[async_trait]
impl<'a> MutFileService for DefaultMutFileService<'a> {
  async fn create(&self, data: &[u8], file_path: &str, file_name: Option<&str>) -> Result<FileName, ServiceError<MutFileServiceError>> {
    Ok(self.repository.create(data, file_path, file_name).await?)
  }

  async fn create_base64(&self, data: &str, file_path: &str, file_name: Option<&str>) -> Result<FileName, ServiceError<MutFileServiceError>> {
    Ok(self.repository.create_base64(data, file_path, file_name).await?)
  }

  async fn delete(&self, uri: &str) -> Result<(), ServiceError<MutFileServiceError>> {
    Ok(self.repository.delete(uri).await?)
  }
}
