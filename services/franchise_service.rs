pub mod mut_franchise_service;

use std::fmt::{Display, Formatter};

use async_trait::async_trait;

use domain::entities::franchise::Franchise;
use domain::enums::language::Language;
use domain::items_total::ItemsTotal;
use domain::pagination::Pagination;

use crate::traits::service_error::ServiceError;

#[async_trait]
pub trait FranchiseService: Send + Sync {
  async fn get(&self, language: Language, pagination: Pagination) -> Result<ItemsTotal<Franchise>, ServiceError<FranchiseServiceError>>;
  async fn get_by_id(&self, id: u32, language: Language) -> Result<Option<Franchise>, ServiceError<FranchiseServiceError>>;
  async fn get_by_name(&self, name: &str, language: Language, pagination: Pagination) -> Result<ItemsTotal<Franchise>, ServiceError<FranchiseServiceError>>;
}

pub enum FranchiseServiceError {}

impl Display for FranchiseServiceError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "")
  }
}

