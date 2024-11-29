use std::sync::Arc;

use async_trait::async_trait;

use domain::entities::franchise::Franchise;
use domain::enums::language::Language;
use domain::items_total::ItemsTotal;
use domain::pagination::Pagination;
use repositories::franchise_repository::FranchiseRepository;
use services::franchise_service::{FranchiseService, FranchiseServiceError};
use services::traits::service_error::ServiceError;

pub struct DefaultFranchiseService<'a> {
  franchise_repository: Arc<dyn FranchiseRepository + 'a>,
}

impl<'a> DefaultFranchiseService<'a> {
  pub fn new(franchise_repository: Arc<dyn FranchiseRepository + 'a>) -> DefaultFranchiseService<'a> {
    DefaultFranchiseService { franchise_repository }
  }
}

#[async_trait]
impl FranchiseService for DefaultFranchiseService<'_> {
  async fn get(
    &self,
    language: Language,
    pagination: Pagination,
  ) -> Result<ItemsTotal<Franchise>, ServiceError<FranchiseServiceError>> {
    Ok(self.franchise_repository.get(language, pagination).await?)
  }

  async fn get_by_id(
    &self,
    id: u32,
    language: Language,
  ) -> Result<Option<Franchise>, ServiceError<FranchiseServiceError>> {
    Ok(self.franchise_repository.get_by_id(id, language).await?)
  }

  async fn get_by_name(
    &self,
    name: &str,
    language: Language,
    pagination: Pagination,
  ) -> Result<ItemsTotal<Franchise>, ServiceError<FranchiseServiceError>> {
    Ok(
      self
        .franchise_repository
        .get_by_name(name, language, pagination)
        .await?,
    )
  }
}
