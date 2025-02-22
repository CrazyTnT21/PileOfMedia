use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;

use domain::entities::theme::Theme;
use domain::entities::theme::create_partial_theme::{CreatePartialTheme, CreatePartialThemeTranslation};
use domain::entities::theme::create_theme::{CreateTheme, CreateThemeTranslation};
use domain::enums::language::Language;
use repositories::theme_repository::ThemeRepository;
use repositories::theme_repository::mut_theme_repository::MutThemeRepository;
use services::theme_service::mut_theme_service::{MutThemeService, MutThemeServiceError};
use services::traits::service_error::ServiceError;
use services::traits::service_error::ServiceError::ClientError;

pub struct DefaultMutThemeService<'a> {
  theme_repository: Arc<dyn ThemeRepository + 'a>,
  mut_theme_repository: Arc<dyn MutThemeRepository + 'a>,
}

impl<'a> DefaultMutThemeService<'a> {
  pub fn new(
    theme_repository: Arc<dyn ThemeRepository + 'a>,
    mut_theme_repository: Arc<dyn MutThemeRepository + 'a>,
  ) -> DefaultMutThemeService<'a> {
    DefaultMutThemeService {
      theme_repository,
      mut_theme_repository,
    }
  }
}

#[async_trait]
impl MutThemeService for DefaultMutThemeService<'_> {
  async fn create(&self, item: CreateTheme) -> Result<Theme, ServiceError<MutThemeServiceError>> {
    self.validate_create(&item).await?;
    let translations = self.transform_translations(item.translations).await?;

    let partial_theme = CreatePartialTheme { translations };
    Ok(self.mut_theme_repository.create(partial_theme).await?)
  }

  async fn delete(&self, ids: &[u32]) -> Result<(), ServiceError<MutThemeServiceError>> {
    self.validate_delete(ids).await?;
    Ok(self.mut_theme_repository.delete(ids).await?)
  }
}

impl DefaultMutThemeService<'_> {
  async fn validate_delete(&self, ids: &[u32]) -> Result<(), ServiceError<MutThemeServiceError>> {
    if ids.is_empty() {
      return Err(ClientError(MutThemeServiceError::NoIdsProvided));
    }
    let existing = self.theme_repository.filter_existing(ids).await?;
    if existing.len() != ids.len() {
      let non_existent = filter_non_existent(ids, &existing);
      return Err(ClientError(MutThemeServiceError::NonExistent(non_existent)));
    };

    Ok(())
  }
  async fn validate_translations(
    &self,
    translations: &HashMap<Language, CreateThemeTranslation>,
  ) -> Result<(), ServiceError<MutThemeServiceError>> {
    if translations.is_empty() {
      return Err(ClientError(MutThemeServiceError::NoTranslationsProvided));
    }
    for item in translations.values() {
      if item.name.is_empty() {
        return Err(ClientError(MutThemeServiceError::InvalidName(item.name.clone())));
      }
    }
    Ok(())
  }
  async fn transform_translations(
    &self,
    translations: HashMap<Language, CreateThemeTranslation>,
  ) -> Result<HashMap<Language, CreatePartialThemeTranslation>, ServiceError<MutThemeServiceError>> {
    let mut hash_map: HashMap<Language, CreatePartialThemeTranslation> = HashMap::new();
    for (language, translation) in translations {
      hash_map.insert(language, CreatePartialThemeTranslation { name: translation.name });
    }
    Ok(hash_map)
  }
  async fn validate_create(&self, item: &CreateTheme) -> Result<(), ServiceError<MutThemeServiceError>> {
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
