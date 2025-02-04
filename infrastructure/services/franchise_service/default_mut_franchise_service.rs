use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;

use domain::entities::franchise::create_franchise::{CreateFranchise, CreateFranchiseTranslation};
use domain::entities::franchise::create_partial_franchise::{
  CreatePartialFranchise, CreatePartialFranchiseTranslation,
};
use domain::entities::franchise::Franchise;
use domain::enums::language::Language;
use repositories::franchise_repository::mut_franchise_repository::MutFranchiseRepository;
use repositories::franchise_repository::FranchiseRepository;
use services::franchise_service::mut_franchise_service::{MutFranchiseService, MutFranchiseServiceError};
use services::traits::service_error::ServiceError;
use services::traits::service_error::ServiceError::ClientError;

pub struct DefaultMutFranchiseService<'a> {
  franchise_repository: Arc<dyn FranchiseRepository + 'a>,
  mut_franchise_repository: Arc<dyn MutFranchiseRepository + 'a>,
}

impl<'a> DefaultMutFranchiseService<'a> {
  pub fn new(
    franchise_repository: Arc<dyn FranchiseRepository + 'a>,
    mut_franchise_repository: Arc<dyn MutFranchiseRepository + 'a>,
  ) -> DefaultMutFranchiseService<'a> {
    DefaultMutFranchiseService {
      franchise_repository,
      mut_franchise_repository,
    }
  }
}

#[async_trait]
impl MutFranchiseService for DefaultMutFranchiseService<'_> {
  async fn create(&self, item: CreateFranchise) -> Result<Franchise, ServiceError<MutFranchiseServiceError>> {
    self.validate_create(&item).await?;
    let translations = self.transform_translations(item.translations).await?;

    let partial_franchise = CreatePartialFranchise { translations };
    Ok(self.mut_franchise_repository.create(partial_franchise).await?)
  }

  async fn delete(&self, ids: &[u32]) -> Result<(), ServiceError<MutFranchiseServiceError>> {
    self.validate_delete(ids).await?;
    Ok(self.mut_franchise_repository.delete(ids).await?)
  }
}

impl DefaultMutFranchiseService<'_> {
  async fn validate_delete(&self, ids: &[u32]) -> Result<(), ServiceError<MutFranchiseServiceError>> {
    if ids.is_empty() {
      return Err(ClientError(MutFranchiseServiceError::NoIdsProvided));
    }
    let existing = self.franchise_repository.filter_existing(ids).await?;
    if existing.len() != ids.len() {
      let non_existent = filter_non_existent(ids, &existing);
      return Err(ClientError(MutFranchiseServiceError::NonExistent(non_existent)));
    };

    Ok(())
  }
  async fn validate_translations(
    &self,
    translations: &HashMap<Language, CreateFranchiseTranslation>,
  ) -> Result<(), ServiceError<MutFranchiseServiceError>> {
    if translations.is_empty() {
      return Err(ClientError(MutFranchiseServiceError::NoTranslationsProvided));
    }
    for item in translations.values() {
      if item.name.is_empty() {
        return Err(ClientError(MutFranchiseServiceError::InvalidName(item.name.clone())));
      }
    }
    Ok(())
  }
  async fn transform_translations(
    &self,
    translations: HashMap<Language, CreateFranchiseTranslation>,
  ) -> Result<HashMap<Language, CreatePartialFranchiseTranslation>, ServiceError<MutFranchiseServiceError>> {
    let mut hash_map: HashMap<Language, CreatePartialFranchiseTranslation> = HashMap::new();
    for (language, translation) in translations {
      hash_map.insert(language, CreatePartialFranchiseTranslation { name: translation.name });
    }
    Ok(hash_map)
  }
  async fn validate_create(&self, item: &CreateFranchise) -> Result<(), ServiceError<MutFranchiseServiceError>> {
    self.validate_translations(&item.translations).await?;
    Ok(())
  }
}

fn filter_non_existent(items: &[u32], existing: &[u32]) -> Vec<u32> {
  items
    .iter()
    .filter_map(|x| existing.iter().find(|y| **y == *x).map_or(Some(*x), |_| None))
    .collect()
}
