use std::cmp::Ordering;
use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;

use domain::entities::book::Book;
use domain::entities::book::create_book::{CreateBook, CreateBookTranslation, CreateCover};
use domain::entities::book::create_partial_book::{CreatePartialBook, CreatePartialBookTranslation};
use domain::enums::language::Language;
use repositories::book_repository::book_theme_repository::BookThemeRepository;
use repositories::book_repository::BookRepository;
use repositories::book_repository::mut_book_repository::MutBookRepository;
use repositories::character_repository::CharacterRepository;
use repositories::franchise_repository::FranchiseRepository;
use repositories::genre_repository::GenreRepository;
use repositories::person_repository::PersonRepository;
use repositories::role_repository::RoleRepository;
use repositories::theme_repository::ThemeRepository;
use services::book_service::mut_book_service::{MutBookService, MutBookServiceError};
use services::book_service::mut_book_service::MutBookServiceError::OtherError;
use services::image_service::mut_image_service::MutImageService;
use services::traits::service_error::{ServiceError};
use services::traits::service_error::ServiceError::ClientError;

use crate::services::map_server_error;

pub struct DefaultMutBookService<'a> {
  default_language: Language,
  book_repository: Arc<dyn BookRepository + 'a>,
  mut_book_repository: Arc<dyn MutBookRepository + 'a>,
  mut_image_service: Arc<dyn MutImageService + 'a>,
  franchise_repository: Arc<dyn FranchiseRepository + 'a>,
  theme_repository: Arc<dyn ThemeRepository + 'a>,
  genre_repository: Arc<dyn GenreRepository + 'a>,
  character_repository: Arc<dyn CharacterRepository + 'a>,
  person_repository: Arc<dyn PersonRepository + 'a>,
  role_repository: Arc<dyn RoleRepository + 'a>,
}

impl<'a> DefaultMutBookService<'a> {
  pub fn new( //TODO: Refactor dependencies into ValidationService
              default_language: Language,
              book_repository: Arc<dyn BookRepository + 'a>,
              mut_book_repository: Arc<dyn MutBookRepository + 'a>,
              mut_image_service: Arc<dyn MutImageService + 'a>,
              franchise_repository: Arc<dyn FranchiseRepository + 'a>,
              theme_repository: Arc<dyn ThemeRepository + 'a>,
              genre_repository: Arc<dyn GenreRepository + 'a>,
              character_repository: Arc<dyn CharacterRepository + 'a>,
              person_repository: Arc<dyn PersonRepository + 'a>,
              role_repository: Arc<dyn RoleRepository + 'a>, ) -> DefaultMutBookService<'a> {
    DefaultMutBookService {
      book_repository,
      default_language,
      mut_book_repository,
      mut_image_service,
      franchise_repository,
      theme_repository,
      genre_repository,
      character_repository,
      person_repository,
      role_repository,
    }
  }
}

#[async_trait]
impl<'a> MutBookService for DefaultMutBookService<'a> {
  async fn create(&self, item: CreateBook) -> Result<Book, ServiceError<MutBookServiceError>> {
    self.validate_create(&item).await?;
    let translations = self.transform_translations(item.translations).await?;

    let partial_book = CreatePartialBook {
      chapters: item.chapters,
      pages: item.pages,
      words: item.words,
      published: item.published,
      franchise: item.franchise,
      translations,
      genres: item.genres,
      themes: item.themes,
      characters: item.characters,
      involved: item.involved,
    };
    self.mut_book_repository.create(partial_book).await.map_err(map_server_error)
  }

  async fn delete(&self, ids: &[u32]) -> Result<(), ServiceError<MutBookServiceError>> {
    self.validate_delete(ids).await?;
    self.mut_book_repository.delete(ids).await.map_err(map_server_error)
  }
}

impl<'a> DefaultMutBookService<'a> {
  async fn validate_delete(&self, ids: &[u32]) -> Result<(), ServiceError<MutBookServiceError>> {
    if ids.is_empty() {
      return Err(ClientError(MutBookServiceError::NoIdsProvided));
    }

    let existing = self.book_repository.filter_existing(ids).await.map_err(map_server_error)?;
    if existing.len() != ids.len() {
      let non_existent_books = filter_non_existent(ids, &existing);
      return Err(ClientError(MutBookServiceError::NonExistentBooks(non_existent_books)));
    };
    //TODO: Validate UserBook
    Ok(())
  }
  async fn validate_translations(&self, translations: &HashMap<Language, CreateBookTranslation>, default_language: &Language) -> Result<(), ServiceError<MutBookServiceError>> {
    if translations.is_empty() {
      return Err(ClientError(MutBookServiceError::NoTranslationsProvided));
    }
    if !translations.contains_key(default_language) {
      return Err(ClientError(MutBookServiceError::NoTranslationInLanguageProvided(default_language.clone())));
    }
    for (current_language, item) in translations {
      if item.title.is_empty() {
        return Err(ClientError(MutBookServiceError::InvalidTitle(item.title.clone())));
      }
      if let Some(description) = &item.description {
        if description.is_empty() {
          return Err(ClientError(MutBookServiceError::InvalidDescription(description.clone())));
        }
      }
      if let CreateCover::ReuseFromLanguage(language) = item.cover {
        let valid_reuse = match translations.get(&language) {
          None => false,
          Some(value) => match value.cover {
            CreateCover::Image(_) => true,
            CreateCover::ReuseFromLanguage(_) => false
          }
        };
        if *current_language == language || !valid_reuse {
          return Err(ClientError(MutBookServiceError::NonExistentTranslationCover(language)));
        }
      }
    }
    Ok(())
  }
  async fn transform_translations(&self, translations: HashMap<Language, CreateBookTranslation>) -> Result<HashMap<Language, CreatePartialBookTranslation>, ServiceError<MutBookServiceError>> {
    let mut hash_map: HashMap<Language, CreatePartialBookTranslation> = HashMap::new();
    let mut translations: Vec<(Language, CreateBookTranslation)> = translations.into_iter().collect();
    sort_translations(&mut translations);
    for (language, translation) in translations {
      let cover = match translation.cover {
        CreateCover::Image(image) => self.mut_image_service
          .create(image)
          .await
          .map_err(|x| ClientError(OtherError(Box::new(x))))?,
        CreateCover::ReuseFromLanguage(lang) => hash_map
          .get(&lang)
          .expect("Translations are sorted. reuse_from_language should appear last")
          .clone()
          .cover
      };

      hash_map.insert(language, CreatePartialBookTranslation {
        title: translation.title,
        description: translation.description,
        cover,
      });
    }
    Ok(hash_map)
  }
  async fn validate_create(&self, item: &CreateBook) -> Result<(), ServiceError<MutBookServiceError>> {
    if let Some(franchise_id) = item.franchise {
      let ids = self.franchise_repository.filter_existing(&[franchise_id]).await.map_err(map_server_error)?;
      if ids.is_empty() {
        return Err(ClientError(MutBookServiceError::NonExistentFranchise(franchise_id)));
      }
    }
    if !item.themes.is_empty() {
      let existing_themes = self.theme_repository.filter_existing(&item.themes).await.map_err(map_server_error)?;
      if item.themes.len() != existing_themes.len() {
        let non_existent_themes = filter_non_existent(&item.themes, &existing_themes);
        return Err(ClientError(MutBookServiceError::NonExistentThemes(non_existent_themes)));
      }
    }
    if !item.genres.is_empty() {
      let existing_genres = self.genre_repository.filter_existing(&item.genres).await.map_err(map_server_error)?;
      if item.genres.len() != existing_genres.len() {
        let non_existent_genres = filter_non_existent(&item.genres, &existing_genres);
        return Err(ClientError(MutBookServiceError::NonExistentGenres(non_existent_genres)));
      }
    }
    if !item.characters.is_empty() {
      let existing_characters = self.character_repository.filter_existing(&item.characters).await.map_err(map_server_error)?;
      if item.characters.len() != existing_characters.len() {
        let non_existent_characters = filter_non_existent(&item.characters, &existing_characters);
        return Err(ClientError(MutBookServiceError::NonExistentCharacters(non_existent_characters)));
      }
    }
    let people: Vec<u32> = item.involved.iter().map(|x| x.person_id).collect();
    if !people.is_empty() {
      let existing_people = self.person_repository.filter_existing(&people).await.map_err(map_server_error)?;
      if people.len() != existing_people.len() {
        let non_existent_people = filter_non_existent(&people, &existing_people);
        return Err(ClientError(MutBookServiceError::NonExistentPeople(non_existent_people)));
      }
    }
    let roles: Vec<u32> = item.involved.iter().map(|x| x.role_id).collect();
    if !roles.is_empty() {
      let existing_roles = self.role_repository.filter_existing(&roles).await.map_err(map_server_error)?;
      if roles.len() != existing_roles.len() {
        let non_existent_roles = filter_non_existent(&roles, &existing_roles);
        return Err(ClientError(MutBookServiceError::NonExistentRoles(non_existent_roles)));
      }
    }
    self.validate_translations(&item.translations, &self.default_language).await?;
    Ok(())
  }
}

fn filter_non_existent(items: &[u32], existing: &[u32]) -> Vec<u32> {
  items.iter().filter_map(|x|
    existing.iter()
      .find(|y| **y == *x)
      .map(|_| None)
      .unwrap_or(Some(*x))
  ).collect()
}

fn sort_translations(translations: &mut Vec<(Language, CreateBookTranslation)>) {
  translations.sort_by(|(_, x), ((_, y))| {
    let x_reuse = match x.cover {
      CreateCover::Image(_) => false,
      CreateCover::ReuseFromLanguage(_) => true
    };
    let y_reuse = match y.cover {
      CreateCover::Image(_) => false,
      CreateCover::ReuseFromLanguage(_) => true
    };
    if x_reuse && !y_reuse {
      return Ordering::Greater;
    }
    if !x_reuse && y_reuse {
      return Ordering::Less;
    }
    return Ordering::Equal;
  });
}
