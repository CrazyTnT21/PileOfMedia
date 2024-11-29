use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;

use domain::entities::role::create_partial_role::{CreatePartialRole, CreatePartialRoleTranslation};
use domain::entities::role::create_role::{CreateRole, CreateRoleTranslation};
use domain::entities::role::Role;
use domain::enums::language::Language;
use repositories::role_repository::mut_role_repository::MutRoleRepository;
use repositories::role_repository::RoleRepository;
use services::role_service::mut_role_service::{MutRoleService, MutRoleServiceError};
use services::traits::service_error::ServiceError;
use services::traits::service_error::ServiceError::ClientError;

pub struct DefaultMutRoleService<'a> {
  default_language: Language,
  role_repository: Arc<dyn RoleRepository + 'a>,
  mut_role_repository: Arc<dyn MutRoleRepository + 'a>,
}

impl<'a> DefaultMutRoleService<'a> {
  pub fn new(
    default_language: Language,
    role_repository: Arc<dyn RoleRepository + 'a>,
    mut_role_repository: Arc<dyn MutRoleRepository + 'a>,
  ) -> DefaultMutRoleService<'a> {
    DefaultMutRoleService {
      default_language,
      role_repository,
      mut_role_repository,
    }
  }
}

#[async_trait]
impl MutRoleService for DefaultMutRoleService<'_> {
  async fn create(&self, item: CreateRole) -> Result<Role, ServiceError<MutRoleServiceError>> {
    self.validate_create(&item).await?;
    let translations = self.transform_translations(item.translations).await?;

    let partial_role = CreatePartialRole { translations };
    Ok(self.mut_role_repository.create(partial_role).await?)
  }

  async fn delete(&self, ids: &[u32]) -> Result<(), ServiceError<MutRoleServiceError>> {
    self.validate_delete(ids).await?;
    Ok(self.mut_role_repository.delete(ids).await?)
  }
}

impl<'a> DefaultMutRoleService<'a> {
  async fn validate_delete(&self, ids: &[u32]) -> Result<(), ServiceError<MutRoleServiceError>> {
    if ids.is_empty() {
      return Err(ClientError(MutRoleServiceError::NoIdsProvided));
    }
    let existing = self.role_repository.filter_existing(ids).await?;
    if existing.len() != ids.len() {
      let non_existent = filter_non_existent(ids, &existing);
      return Err(ClientError(MutRoleServiceError::NonExistent(non_existent)));
    };

    Ok(())
  }
  async fn validate_translations(
    &self,
    translations: &HashMap<Language, CreateRoleTranslation>,
    default_language: &Language,
  ) -> Result<(), ServiceError<MutRoleServiceError>> {
    if translations.is_empty() {
      return Err(ClientError(MutRoleServiceError::NoTranslationsProvided));
    }
    if !translations.contains_key(default_language) {
      return Err(ClientError(MutRoleServiceError::NoTranslationInLanguageProvided(
        *default_language,
      )));
    }
    for item in translations.values() {
      if item.name.is_empty() {
        return Err(ClientError(MutRoleServiceError::InvalidName(item.name.clone())));
      }
    }
    Ok(())
  }
  async fn transform_translations(
    &self,
    translations: HashMap<Language, CreateRoleTranslation>,
  ) -> Result<HashMap<Language, CreatePartialRoleTranslation>, ServiceError<MutRoleServiceError>> {
    let mut hash_map: HashMap<Language, CreatePartialRoleTranslation> = HashMap::new();
    for (language, translation) in translations {
      hash_map.insert(language, CreatePartialRoleTranslation { name: translation.name });
    }
    Ok(hash_map)
  }
  async fn validate_create(&self, item: &CreateRole) -> Result<(), ServiceError<MutRoleServiceError>> {
    self
      .validate_translations(&item.translations, &self.default_language)
      .await?;
    Ok(())
  }
}

fn filter_non_existent(items: &[u32], existing: &[u32]) -> Vec<u32> {
  items
    .iter()
    .filter_map(|x| existing.iter().find(|y| **y == *x).map_or(Some(*x), |_| None))
    .collect()
}
