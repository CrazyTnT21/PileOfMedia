use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;

use domain::entities::genre::Genre;
use domain::entities::genre::create_genre::{CreateGenre, CreateGenreTranslation};
use domain::entities::genre::create_partial_genre::{CreatePartialGenre, CreatePartialGenreTranslation};
use domain::enums::language::Language;
use repositories::genre_repository::GenreRepository;
use repositories::genre_repository::mut_genre_repository::MutGenreRepository;
use services::genre_service::mut_genre_service::{MutGenreService, MutGenreServiceError};
use services::traits::service_error::ServiceError;
use services::traits::service_error::ServiceError::ClientError;

pub struct DefaultMutGenreService<'a> {
  genre_repository: Arc<dyn GenreRepository + 'a>,
  mut_genre_repository: Arc<dyn MutGenreRepository + 'a>,
}

impl<'a> DefaultMutGenreService<'a> {
  pub fn new(
    genre_repository: Arc<dyn GenreRepository + 'a>,
    mut_genre_repository: Arc<dyn MutGenreRepository + 'a>,
  ) -> DefaultMutGenreService<'a> {
    DefaultMutGenreService {
      genre_repository,
      mut_genre_repository,
    }
  }
}

#[async_trait]
impl MutGenreService for DefaultMutGenreService<'_> {
  async fn create(&self, item: CreateGenre) -> Result<Genre, ServiceError<MutGenreServiceError>> {
    self.validate_create(&item).await?;
    let translations = self.transform_translations(item.translations).await?;

    let partial_genre = CreatePartialGenre { translations };
    Ok(self.mut_genre_repository.create(partial_genre).await?)
  }

  async fn delete(&self, ids: &[u32]) -> Result<(), ServiceError<MutGenreServiceError>> {
    self.validate_delete(ids).await?;
    Ok(self.mut_genre_repository.delete(ids).await?)
  }
}

impl DefaultMutGenreService<'_> {
  async fn validate_delete(&self, ids: &[u32]) -> Result<(), ServiceError<MutGenreServiceError>> {
    if ids.is_empty() {
      return Err(ClientError(MutGenreServiceError::NoIdsProvided));
    }
    let existing = self.genre_repository.filter_existing(ids).await?;
    if existing.len() != ids.len() {
      let non_existent = filter_non_existent(ids, &existing);
      return Err(ClientError(MutGenreServiceError::NonExistent(non_existent)));
    };

    Ok(())
  }
  async fn validate_translations(
    &self,
    translations: &HashMap<Language, CreateGenreTranslation>,
  ) -> Result<(), ServiceError<MutGenreServiceError>> {
    if translations.is_empty() {
      return Err(ClientError(MutGenreServiceError::NoTranslationsProvided));
    }
    for item in translations.values() {
      if item.name.is_empty() {
        return Err(ClientError(MutGenreServiceError::InvalidName(item.name.clone())));
      }
    }
    Ok(())
  }
  async fn transform_translations(
    &self,
    translations: HashMap<Language, CreateGenreTranslation>,
  ) -> Result<HashMap<Language, CreatePartialGenreTranslation>, ServiceError<MutGenreServiceError>> {
    let mut hash_map: HashMap<Language, CreatePartialGenreTranslation> = HashMap::new();
    for (language, translation) in translations {
      hash_map.insert(language, CreatePartialGenreTranslation { name: translation.name });
    }
    Ok(hash_map)
  }
  async fn validate_create(&self, item: &CreateGenre) -> Result<(), ServiceError<MutGenreServiceError>> {
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
