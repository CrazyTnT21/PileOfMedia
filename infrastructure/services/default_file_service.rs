use async_trait::async_trait;

use repositories::file_repository::FileRepository;
use services::file_service::FileService;
use services::traits::service_error::ServiceError;
use crate::services::map_server_error;

pub struct DefaultFileService<'a> {
  repository: &'a dyn FileRepository,
}

impl<'a> DefaultFileService<'a> {
  pub fn new(repository: &'a impl FileRepository) -> DefaultFileService {
    DefaultFileService { repository }
  }
}

#[async_trait]
impl<'a> FileService for DefaultFileService<'a> {
  async fn create(&self, data: &[u8], file_path: &str, file_name: Option<&str>) -> Result<String, ServiceError> {
    self.repository.create(data, file_path, file_name).await.map_err(map_server_error)
  }

  async fn create_base64(&self, data: &str, file_path: &str, file_name: Option<&str>) -> Result<String, ServiceError> {
    self.repository.create_base64(data, file_path, file_name).await.map_err(map_server_error)
  }
}
