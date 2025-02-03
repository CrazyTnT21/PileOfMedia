use crate::controllers::DEFAULT_LANGUAGE;
use crate::implementations::{
  get_book_character_repository, get_book_character_service, get_book_genre_repository, get_book_genre_service,
  get_book_involved_repository, get_book_involved_service, get_book_repository, get_book_service,
  get_book_theme_repository, get_book_theme_service, get_character_repository, get_file_repository,
  get_franchise_repository, get_genre_repository, get_image_repository, get_mut_book_character_repository,
  get_mut_book_character_service, get_mut_book_genre_repository, get_mut_book_genre_service,
  get_mut_book_involved_repository, get_mut_book_involved_service, get_mut_book_repository, get_mut_book_service,
  get_mut_book_theme_repository, get_mut_book_theme_service, get_mut_file_repository, get_mut_file_service,
  get_mut_image_repository, get_mut_image_service, get_person_repository, get_role_repository, get_theme_repository,
};
use services::book_service::book_character_service::mut_book_character_service::MutBookCharacterService;
use services::book_service::book_character_service::BookCharacterService;
use services::book_service::book_genre_service::mut_book_genre_service::MutBookGenreService;
use services::book_service::book_genre_service::BookGenreService;
use services::book_service::book_involved_service::mut_book_involved_service::MutBookInvolvedService;
use services::book_service::book_involved_service::BookInvolvedService;
use services::book_service::book_theme_service::mut_book_theme_service::MutBookThemeService;
use services::book_service::book_theme_service::BookThemeService;
use services::book_service::mut_book_service::MutBookService;
use services::book_service::BookService;
use std::sync::Arc;
use tokio_postgres::{Client, Transaction};

pub fn get_genre_service(connection: &Client) -> impl BookGenreService + '_ {
  let image_repository = Arc::new(get_image_repository(connection));
  let franchise_repository = Arc::new(get_franchise_repository(connection, DEFAULT_LANGUAGE));
  let book_repository = Arc::new(get_book_repository(
    connection,
    DEFAULT_LANGUAGE,
    image_repository.clone(),
    franchise_repository,
  ));
  let genre_repository = Arc::new(get_genre_repository(connection));
  let repository = Arc::new(get_book_genre_repository(connection, book_repository, genre_repository));
  get_book_genre_service(repository)
}

pub fn get_mut_genre_service<'a>(
  transaction: &'a Transaction<'a>,
  client: &'a Client,
) -> impl MutBookGenreService + 'a {
  let image_repository = Arc::new(get_image_repository(client));
  let franchise_repository = Arc::new(get_franchise_repository(client, DEFAULT_LANGUAGE));
  let book_repository = Arc::new(get_book_repository(
    client,
    DEFAULT_LANGUAGE,
    image_repository.clone(),
    franchise_repository,
  ));
  let genre_repository = Arc::new(get_genre_repository(client));
  let book_genre_repository = get_book_genre_repository(client, book_repository.clone(), genre_repository.clone());
  let repository = get_mut_book_genre_repository(transaction);
  get_mut_book_genre_service(
    book_repository,
    Arc::new(book_genre_repository),
    Arc::new(repository),
    genre_repository,
  )
}

pub fn get_theme_service(connection: &Client) -> impl BookThemeService + '_ {
  let image_repository = Arc::new(get_image_repository(connection));
  let franchise_repository = Arc::new(get_franchise_repository(connection, DEFAULT_LANGUAGE));
  let book_repository = Arc::new(get_book_repository(
    connection,
    DEFAULT_LANGUAGE,
    image_repository.clone(),
    franchise_repository,
  ));
  let theme_repository = Arc::new(get_theme_repository(connection));
  let repository = Arc::new(get_book_theme_repository(connection, book_repository, theme_repository));
  get_book_theme_service(repository)
}

pub fn get_mut_theme_service<'a>(
  transaction: &'a Transaction<'a>,
  client: &'a Client,
) -> impl MutBookThemeService + 'a {
  let image_repository = Arc::new(get_image_repository(client));
  let franchise_repository = Arc::new(get_franchise_repository(client, DEFAULT_LANGUAGE));
  let book_repository = Arc::new(get_book_repository(
    client,
    DEFAULT_LANGUAGE,
    image_repository.clone(),
    franchise_repository,
  ));
  let theme_repository = Arc::new(get_theme_repository(client));
  let book_theme_repository = get_book_theme_repository(client, book_repository.clone(), theme_repository.clone());
  let repository = get_mut_book_theme_repository(transaction);
  get_mut_book_theme_service(
    book_repository,
    Arc::new(book_theme_repository),
    Arc::new(repository),
    theme_repository,
  )
}

pub fn get_character_service(connection: &Client) -> impl BookCharacterService + '_ {
  let image_repository = Arc::new(get_image_repository(connection));
  let franchise_repository = Arc::new(get_franchise_repository(connection, DEFAULT_LANGUAGE));
  let book_repository = Arc::new(get_book_repository(
    connection,
    DEFAULT_LANGUAGE,
    image_repository.clone(),
    franchise_repository,
  ));
  let character_repository = Arc::new(get_character_repository(connection, image_repository));
  let repository = Arc::new(get_book_character_repository(
    connection,
    book_repository,
    character_repository,
  ));
  get_book_character_service(repository)
}

pub fn get_mut_character_service<'a>(
  transaction: &'a Transaction<'a>,
  client: &'a Client,
) -> impl MutBookCharacterService + 'a {
  let image_repository = Arc::new(get_image_repository(client));
  let franchise_repository = Arc::new(get_franchise_repository(client, DEFAULT_LANGUAGE));
  let book_repository = Arc::new(get_book_repository(
    client,
    DEFAULT_LANGUAGE,
    image_repository.clone(),
    franchise_repository,
  ));
  let character_repository = Arc::new(get_character_repository(client, image_repository));
  let book_character_repository =
    get_book_character_repository(client, book_repository.clone(), character_repository.clone());
  let repository = get_mut_book_character_repository(transaction);
  get_mut_book_character_service(
    book_repository,
    Arc::new(book_character_repository),
    Arc::new(repository),
    character_repository,
  )
}

pub fn get_involved_service(connection: &Client) -> impl BookInvolvedService + '_ {
  let image_repository = Arc::new(get_image_repository(connection));
  let franchise_repository = Arc::new(get_franchise_repository(connection, DEFAULT_LANGUAGE));
  let book_repository = Arc::new(get_book_repository(
    connection,
    DEFAULT_LANGUAGE,
    image_repository.clone(),
    franchise_repository,
  ));
  let person_repository = Arc::new(get_person_repository(
    connection,
    DEFAULT_LANGUAGE,
    image_repository.clone(),
  ));
  let role_repository = Arc::new(get_role_repository(connection, DEFAULT_LANGUAGE));
  let repository = Arc::new(get_book_involved_repository(
    connection,
    DEFAULT_LANGUAGE,
    book_repository,
    person_repository,
    role_repository,
  ));
  get_book_involved_service(repository)
}

pub fn get_service(connection: &Client) -> impl BookService + '_ {
  let image_repository = Arc::new(get_image_repository(connection));
  let franchise_repository = Arc::new(get_franchise_repository(connection, DEFAULT_LANGUAGE));
  let repository = get_book_repository(connection, DEFAULT_LANGUAGE, image_repository, franchise_repository);
  get_book_service(Arc::new(repository))
}

pub fn get_mut_involved_service<'a>(
  transaction: &'a Transaction<'a>,
  client: &'a Client,
) -> impl MutBookInvolvedService + 'a {
  let image_repository = Arc::new(get_image_repository(client));
  let franchise_repository = Arc::new(get_franchise_repository(client, DEFAULT_LANGUAGE));
  let book_repository = Arc::new(get_book_repository(
    client,
    DEFAULT_LANGUAGE,
    image_repository.clone(),
    franchise_repository,
  ));
  let role_repository = Arc::new(get_role_repository(client, DEFAULT_LANGUAGE));
  let person_repository = Arc::new(get_person_repository(
    client,
    DEFAULT_LANGUAGE,
    image_repository.clone(),
  ));
  let book_involved_repository = get_book_involved_repository(
    client,
    DEFAULT_LANGUAGE,
    book_repository.clone(),
    person_repository.clone(),
    role_repository.clone(),
  );
  let repository = get_mut_book_involved_repository(transaction);
  get_mut_book_involved_service(
    book_repository,
    Arc::new(book_involved_repository),
    Arc::new(repository),
    person_repository,
    role_repository,
  )
}

//TODO: Refactor
pub fn get_mut_service<'a>(
  transaction: &'a Transaction<'a>,
  client: &'a Client,
  display_path: &'a str,
  path: &'a str,
) -> impl MutBookService + 'a {
  let image_repository = Arc::new(get_image_repository(client));
  let file_repository = Arc::new(get_file_repository());
  let mut_file_repository = Arc::new(get_mut_file_repository());

  let genre_repository = Arc::new(get_genre_repository(client));
  let mut_book_genre_repository = Arc::new(get_mut_book_genre_repository(transaction));
  let mut_file_service = Arc::new(get_mut_file_service(mut_file_repository.clone()));
  let character_repository = Arc::new(get_character_repository(client, image_repository.clone()));
  let mut_book_character_repository = Arc::new(get_mut_book_character_repository(transaction));
  let mut_image_repository = Arc::new(get_mut_image_repository(
    transaction,
    image_repository.clone(),
    mut_file_repository,
    file_repository,
  ));
  let mut_image_service = Arc::new(get_mut_image_service(
    mut_image_repository,
    mut_file_service,
    display_path,
    path,
  ));
  let role_repository = Arc::new(get_role_repository(client, DEFAULT_LANGUAGE));
  let person_repository = Arc::new(get_person_repository(
    client,
    DEFAULT_LANGUAGE,
    image_repository.clone(),
  ));
  let mut_book_involved_repository = Arc::new(get_mut_book_involved_repository(transaction));

  let franchise_repository = Arc::new(get_franchise_repository(client, DEFAULT_LANGUAGE));
  let book_repository = Arc::new(get_book_repository(
    client,
    DEFAULT_LANGUAGE,
    image_repository,
    franchise_repository.clone(),
  ));
  let theme_repository = Arc::new(get_theme_repository(client));
  let mut_book_theme_repository = Arc::new(get_mut_book_theme_repository(transaction));
  let mut_book_repository = Arc::new(get_mut_book_repository(
    transaction,
    DEFAULT_LANGUAGE,
    mut_book_genre_repository,
    mut_book_character_repository,
    mut_book_theme_repository,
    mut_book_involved_repository,
    book_repository.clone(),
  ));
  get_mut_book_service(
    DEFAULT_LANGUAGE,
    book_repository,
    mut_book_repository,
    mut_image_service,
    franchise_repository,
    theme_repository,
    genre_repository,
    character_repository,
    person_repository,
    role_repository,
  )
}
