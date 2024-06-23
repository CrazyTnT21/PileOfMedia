use std::sync::Arc;
use tokio_postgres::{Client, Transaction};
use services::book_service::book_character_service::BookCharacterService;
use services::book_service::book_character_service::mut_book_character_service::MutBookCharacterService;
use services::book_service::book_genre_service::BookGenreService;
use services::book_service::book_genre_service::mut_book_genre_service::MutBookGenreService;
use services::book_service::book_involved_service::BookInvolvedService;
use services::book_service::book_involved_service::mut_book_involved_service::MutBookInvolvedService;
use services::book_service::book_theme_service::BookThemeService;
use services::book_service::book_theme_service::mut_book_theme_service::MutBookThemeService;
use services::book_service::BookService;
use crate::controllers::DEFAULT_LANGUAGE;
use crate::implementations::{get_book_character_repository, get_book_character_service, get_book_genre_repository, get_book_genre_service, get_book_involved_repository, get_book_involved_service, get_book_repository, get_book_service, get_book_theme_repository, get_book_theme_service, get_character_repository, get_genre_repository, get_image_repository, get_mut_book_character_repository, get_mut_book_character_service, get_mut_book_genre_repository, get_mut_book_genre_service, get_mut_book_involved_repository, get_mut_book_involved_service, get_mut_book_theme_repository, get_mut_book_theme_service, get_person_repository, get_role_repository, get_theme_repository};

pub fn get_genre_service(connection: &Client) -> impl BookGenreService + '_ {
  let image_repository = Arc::new(get_image_repository(connection));
  let book_repository = Arc::new(get_book_repository(connection, DEFAULT_LANGUAGE, image_repository.clone()));
  let genre_repository = Arc::new(get_genre_repository(connection, DEFAULT_LANGUAGE));
  let repository = Arc::new(get_book_genre_repository(connection, DEFAULT_LANGUAGE, book_repository, genre_repository));
  get_book_genre_service(repository)
}

pub fn get_mut_genre_service<'a>(transaction: &'a Transaction<'a>, client: &'a Client) -> impl MutBookGenreService + 'a {
  let image_repository = Arc::new(get_image_repository(client));
  let book_repository = Arc::new(get_book_repository(client, DEFAULT_LANGUAGE, image_repository.clone()));
  let genre_repository = Arc::new(get_genre_repository(client, DEFAULT_LANGUAGE));
  let book_genre_repository = get_book_genre_repository(client, DEFAULT_LANGUAGE, book_repository.clone(), genre_repository.clone());
  let repository = get_mut_book_genre_repository(transaction);
  get_mut_book_genre_service(book_repository, Arc::new(book_genre_repository), Arc::new(repository), genre_repository)
}

pub fn get_theme_service(connection: &Client) -> impl BookThemeService + '_ {
  let image_repository = Arc::new(get_image_repository(connection));
  let book_repository = Arc::new(get_book_repository(connection, DEFAULT_LANGUAGE, image_repository.clone()));
  let theme_repository = Arc::new(get_theme_repository(connection, DEFAULT_LANGUAGE));
  let repository = Arc::new(get_book_theme_repository(connection, DEFAULT_LANGUAGE, book_repository, theme_repository));
  get_book_theme_service(repository)
}

pub fn get_mut_theme_service<'a>(transaction: &'a Transaction<'a>, client: &'a Client) -> impl MutBookThemeService + 'a {
  let image_repository = Arc::new(get_image_repository(client));
  let book_repository = Arc::new(get_book_repository(client, DEFAULT_LANGUAGE, image_repository.clone()));
  let theme_repository = Arc::new(get_theme_repository(client, DEFAULT_LANGUAGE));
  let book_theme_repository = get_book_theme_repository(client, DEFAULT_LANGUAGE, book_repository.clone(), theme_repository.clone());
  let repository = get_mut_book_theme_repository(transaction);
  get_mut_book_theme_service(book_repository, Arc::new(book_theme_repository), Arc::new(repository), theme_repository)
}

pub fn get_character_service(connection: &Client) -> impl BookCharacterService + '_ {
  let image_repository = Arc::new(get_image_repository(connection));
  let book_repository = Arc::new(get_book_repository(connection, DEFAULT_LANGUAGE, image_repository.clone()));
  let character_repository = Arc::new(get_character_repository(connection, DEFAULT_LANGUAGE, image_repository));
  let repository = Arc::new(get_book_character_repository(connection, DEFAULT_LANGUAGE, book_repository, character_repository));
  get_book_character_service(repository)
}

pub fn get_mut_character_service<'a>(transaction: &'a Transaction<'a>, client: &'a Client) -> impl MutBookCharacterService + 'a {
  let image_repository = Arc::new(get_image_repository(client));
  let book_repository = Arc::new(get_book_repository(client, DEFAULT_LANGUAGE, image_repository.clone()));
  let character_repository = Arc::new(get_character_repository(client, DEFAULT_LANGUAGE, image_repository));
  let book_character_repository = get_book_character_repository(client, DEFAULT_LANGUAGE, book_repository.clone(), character_repository.clone());
  let repository = get_mut_book_character_repository(transaction);
  get_mut_book_character_service(book_repository, Arc::new(book_character_repository), Arc::new(repository), character_repository)
}

pub fn get_involved_service(connection: &Client) -> impl BookInvolvedService + '_ {
  let image_repository = Arc::new(get_image_repository(connection));
  let book_repository = Arc::new(get_book_repository(connection, DEFAULT_LANGUAGE, image_repository.clone()));
  let person_repository = Arc::new(get_person_repository(connection, DEFAULT_LANGUAGE, image_repository.clone()));
  let role_repository = Arc::new(get_role_repository(connection, DEFAULT_LANGUAGE));
  let repository = Arc::new(get_book_involved_repository(connection, DEFAULT_LANGUAGE, book_repository, person_repository, role_repository));
  get_book_involved_service(repository)
}

pub fn get_service(connection: &Client) -> impl BookService + '_ {
  let image_repository = Arc::new(get_image_repository(connection));
  let repository = get_book_repository(connection, DEFAULT_LANGUAGE, image_repository);
  get_book_service(Arc::new(repository))
}

pub fn get_mut_involved_service<'a>(transaction: &'a Transaction<'a>, client: &'a Client) -> impl MutBookInvolvedService + 'a {
  let image_repository = Arc::new(get_image_repository(client));
  let book_repository = Arc::new(get_book_repository(client, DEFAULT_LANGUAGE, image_repository.clone()));
  let role_repository = Arc::new(get_role_repository(client, DEFAULT_LANGUAGE));
  let person_repository = Arc::new(get_person_repository(client, DEFAULT_LANGUAGE, image_repository.clone()));
  let book_involved_repository = get_book_involved_repository(client, DEFAULT_LANGUAGE, book_repository.clone(), person_repository.clone(), role_repository.clone());
  let repository = get_mut_book_involved_repository(transaction);
  get_mut_book_involved_service(book_repository, Arc::new(book_involved_repository), Arc::new(repository), person_repository, role_repository)
}
