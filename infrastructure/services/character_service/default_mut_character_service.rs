use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;

use domain::entities::character::create_character::{CreateCharacter, CreateCharacterTranslation};
use domain::entities::character::create_partial_character::{
  CreatePartialCharacter, CreatePartialCharacterTranslation,
};
use domain::entities::character::Character;
use domain::enums::language::Language;
use repositories::character_repository::mut_character_repository::MutCharacterRepository;
use repositories::character_repository::CharacterRepository;
use services::character_service::mut_character_service::MutCharacterServiceError::OtherError;
use services::character_service::mut_character_service::{MutCharacterService, MutCharacterServiceError};
use services::image_service::mut_image_service::MutImageService;
use services::traits::service_error::ServiceError;
use services::traits::service_error::ServiceError::{ClientError, ServerError};

pub struct DefaultMutCharacterService<'a> {
  character_repository: Arc<dyn CharacterRepository + 'a>,
  mut_character_repository: Arc<dyn MutCharacterRepository + 'a>,
  mut_image_service: Arc<dyn MutImageService + 'a>,
}

impl<'a> DefaultMutCharacterService<'a> {
  pub fn new(
    character_repository: Arc<dyn CharacterRepository + 'a>,
    mut_character_repository: Arc<dyn MutCharacterRepository + 'a>,
    mut_image_service: Arc<dyn MutImageService + 'a>,
  ) -> DefaultMutCharacterService<'a> {
    DefaultMutCharacterService {
      character_repository,
      mut_character_repository,
      mut_image_service,
    }
  }
}

#[async_trait]
impl MutCharacterService for DefaultMutCharacterService<'_> {
  async fn create(&self, item: CreateCharacter) -> Result<Character, ServiceError<MutCharacterServiceError>> {
    self.validate_create(&item).await?;
    let data = item.character;
    let translations = self.transform_translations(data.translations).await?;
    let image = match item.image {
      None => None,
      Some(value) => Some(self.mut_image_service.create(value).await.map_err(|x| match x {
        ClientError(x) => ClientError(OtherError(Box::new(x))),
        ServerError(x) => ServerError(x),
      })?),
    };
    let partial_character = CreatePartialCharacter {
      birthday: data.birthday,
      height_cm: data.height_cm,
      image,
      translations,
    };
    Ok(self.mut_character_repository.create(partial_character).await?)
  }

  async fn delete(&self, ids: &[u32]) -> Result<(), ServiceError<MutCharacterServiceError>> {
    self.validate_delete(ids).await?;
    Ok(self.mut_character_repository.delete(ids).await?)
  }
}

impl DefaultMutCharacterService<'_> {
  async fn validate_delete(&self, ids: &[u32]) -> Result<(), ServiceError<MutCharacterServiceError>> {
    if ids.is_empty() {
      return Err(ClientError(MutCharacterServiceError::NoIdsProvided));
    }

    let existing = self.character_repository.filter_existing(ids).await?;
    if existing.len() != ids.len() {
      let non_existent_characters = filter_non_existent(ids, &existing);
      return Err(ClientError(MutCharacterServiceError::NonExistentCharacters(
        non_existent_characters,
      )));
    };
    Ok(())
  }
  async fn validate_translations(
    &self,
    translations: &HashMap<Language, CreateCharacterTranslation>,
  ) -> Result<(), ServiceError<MutCharacterServiceError>> {
    if translations.is_empty() {
      return Err(ClientError(MutCharacterServiceError::NoTranslationsProvided));
    }
    for item in translations.values() {
      if item.name.is_empty() {
        return Err(ClientError(MutCharacterServiceError::InvalidName(item.name.clone())));
      }
      if let Some(description) = &item.description {
        if description.is_empty() {
          return Err(ClientError(MutCharacterServiceError::InvalidDescription(
            description.clone(),
          )));
        }
      }
    }
    Ok(())
  }
  async fn transform_translations(
    &self,
    translations: HashMap<Language, CreateCharacterTranslation>,
  ) -> Result<HashMap<Language, CreatePartialCharacterTranslation>, ServiceError<MutCharacterServiceError>> {
    let mut hash_map: HashMap<Language, CreatePartialCharacterTranslation> = HashMap::new();
    for (language, translation) in translations {
      hash_map.insert(
        language,
        CreatePartialCharacterTranslation {
          name: translation.name,
          first_name: translation.first_name,
          last_name: translation.last_name,
          description: translation.description,
        },
      );
    }
    Ok(hash_map)
  }
  async fn validate_create(&self, item: &CreateCharacter) -> Result<(), ServiceError<MutCharacterServiceError>> {
    let data = &item.character;
    self.validate_translations(&data.translations).await?;
    Ok(())
  }
}

fn filter_non_existent(items: &[u32], existing: &[u32]) -> Vec<u32> {
  items
    .iter()
    .filter_map(|x| existing.iter().find(|y| **y == *x).map_or(Some(*x), |_| None))
    .collect()
}
