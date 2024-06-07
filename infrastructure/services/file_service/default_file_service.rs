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
  async fn get(&self, uri: &str) -> Result<Vec<u8>, ServiceError> {
    self.repository.get(uri).await.map_err(map_server_error)
  }
}
