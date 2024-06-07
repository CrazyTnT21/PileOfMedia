use async_trait::async_trait;
use domain::file_name::FileName;

use repositories::file_repository::mut_file_repository::MutFileRepository;
use services::file_service::mut_file_service::MutFileService;
use services::traits::service_error::ServiceError;
use crate::services::map_server_error;

pub struct DefaultMutFileService<'a> {
  repository: &'a dyn MutFileRepository,
}

impl<'a> DefaultMutFileService<'a> {
  pub fn new(repository: &'a impl MutFileRepository) -> DefaultMutFileService {
    DefaultMutFileService { repository }
  }
}

#[async_trait]
impl<'a> MutFileService for DefaultMutFileService<'a> {
  async fn create(&self, data: &[u8], file_path: &str, file_name: Option<&str>) -> Result<FileName, ServiceError> {
    self.repository.create(data, file_path, file_name).await.map_err(map_server_error)
  }

  async fn create_base64(&self, data: &str, file_path: &str, file_name: Option<&str>) -> Result<FileName, ServiceError> {
    self.repository.create_base64(data, file_path, file_name).await.map_err(map_server_error)
  }

  async fn delete(&self, uri: &str) -> Result<(), ServiceError> {
    self.repository.delete(uri).await.map_err(map_server_error)
  }
}
