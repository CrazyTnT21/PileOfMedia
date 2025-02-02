use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;

use domain::entities::person::create_partial_person::{CreatePartialPerson, CreatePartialPersonTranslation};
use domain::entities::person::create_person::{CreatePerson, CreatePersonTranslation};
use domain::entities::person::Person;
use domain::enums::language::Language;
use repositories::person_repository::mut_person_repository::MutPersonRepository;
use repositories::person_repository::PersonRepository;
use services::image_service::mut_image_service::MutImageService;
use services::person_service::mut_person_service::MutPersonServiceError::OtherError;
use services::person_service::mut_person_service::{MutPersonService, MutPersonServiceError};
use services::traits::service_error::ServiceError;
use services::traits::service_error::ServiceError::{ClientError, ServerError};

pub struct DefaultMutPersonService<'a> {
  default_language: Language,
  person_repository: Arc<dyn PersonRepository + 'a>,
  mut_person_repository: Arc<dyn MutPersonRepository + 'a>,
  mut_image_service: Arc<dyn MutImageService + 'a>,
}

impl<'a> DefaultMutPersonService<'a> {
  pub fn new(
    default_language: Language,
    person_repository: Arc<dyn PersonRepository + 'a>,
    mut_person_repository: Arc<dyn MutPersonRepository + 'a>,
    mut_image_service: Arc<dyn MutImageService + 'a>,
  ) -> DefaultMutPersonService<'a> {
    DefaultMutPersonService {
      default_language,
      person_repository,
      mut_person_repository,
      mut_image_service,
    }
  }
}

#[async_trait]
impl MutPersonService for DefaultMutPersonService<'_> {
  async fn create(&self, item: CreatePerson) -> Result<Person, ServiceError<MutPersonServiceError>> {
    self.validate_create(&item).await?;
    let data = item.person;
    let translations = self.transform_translations(data.translations).await?;
    let image = match item.image {
      None => None,
      Some(value) => Some(self.mut_image_service.create(value).await.map_err(|x| match x {
        ClientError(x) => ClientError(OtherError(Box::new(x))),
        ServerError(x) => ServerError(x),
      })?),
    };
    let partial_person = CreatePartialPerson {
      name: data.name,
      first_name: data.first_name,
      last_name: data.last_name,
      birthday: data.birthday,
      height: data.height,
      image,
      translations,
    };
    Ok(self.mut_person_repository.create(partial_person).await?)
  }

  async fn delete(&self, ids: &[u32]) -> Result<(), ServiceError<MutPersonServiceError>> {
    self.validate_delete(ids).await?;
    Ok(self.mut_person_repository.delete(ids).await?)
  }
}

impl DefaultMutPersonService<'_> {
  async fn validate_delete(&self, ids: &[u32]) -> Result<(), ServiceError<MutPersonServiceError>> {
    if ids.is_empty() {
      return Err(ClientError(MutPersonServiceError::NoIdsProvided));
    }

    let existing = self.person_repository.filter_existing(ids).await?;
    if existing.len() != ids.len() {
      let non_existent_people = filter_non_existent(ids, &existing);
      return Err(ClientError(MutPersonServiceError::NonExistentPeople(
        non_existent_people,
      )));
    };
    Ok(())
  }
  async fn validate_translations(
    &self,
    translations: &HashMap<Language, CreatePersonTranslation>,
    default_language: &Language,
  ) -> Result<(), ServiceError<MutPersonServiceError>> {
    if translations.is_empty() {
      return Err(ClientError(MutPersonServiceError::NoTranslationsProvided));
    }
    if !translations.contains_key(default_language) {
      return Err(ClientError(MutPersonServiceError::NoTranslationInLanguageProvided(
        *default_language,
      )));
    }
    for item in translations.values() {
      if let Some(description) = &item.description {
        if description.is_empty() {
          return Err(ClientError(MutPersonServiceError::InvalidDescription(
            description.clone(),
          )));
        }
      }
    }
    Ok(())
  }
  async fn transform_translations(
    &self,
    translations: HashMap<Language, CreatePersonTranslation>,
  ) -> Result<HashMap<Language, CreatePartialPersonTranslation>, ServiceError<MutPersonServiceError>> {
    let mut hash_map: HashMap<Language, CreatePartialPersonTranslation> = HashMap::new();
    for (language, translation) in translations {
      hash_map.insert(
        language,
        CreatePartialPersonTranslation {
          description: translation.description,
        },
      );
    }
    Ok(hash_map)
  }
  async fn validate_create(&self, item: &CreatePerson) -> Result<(), ServiceError<MutPersonServiceError>> {
    let data = &item.person;
    if data.name.is_empty() {
      return Err(ClientError(MutPersonServiceError::InvalidName(data.name.clone())));
    }

    self
      .validate_translations(&data.translations, &self.default_language)
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
