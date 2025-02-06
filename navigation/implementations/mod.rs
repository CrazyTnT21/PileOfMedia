use std::sync::Arc;

use tokio_postgres::{Client, Transaction};

use application::repositories::account_repository::default_account_repository::DefaultAccountRepository;
use application::repositories::account_repository::default_mut_account_repository::DefaultMutAccountRepository;
use application::repositories::book_repository::book_character_repository::default_book_character_repository::DefaultBookCharacterRepository;
use application::repositories::book_repository::book_character_repository::default_mut_book_character_repository::DefaultMutBookCharacterRepository;
use application::repositories::book_repository::book_genre_repository::default_book_genre_repository::DefaultBookGenreRepository;
use application::repositories::book_repository::book_genre_repository::default_mut_book_genre_repository::DefaultMutBookGenreRepository;
use application::repositories::book_repository::book_involved_repository::default_book_involved_repository::DefaultBookInvolvedRepository;
use application::repositories::book_repository::book_involved_repository::default_mut_book_involved_repository::DefaultMutBookInvolvedRepository;
use application::repositories::book_repository::book_theme_repository::default_book_theme_repository::DefaultBookThemeRepository;
use application::repositories::book_repository::book_theme_repository::default_mut_book_theme_repository::DefaultMutBookThemeRepository;
use application::repositories::book_repository::default_book_repository::DefaultBookRepository;
use application::repositories::book_repository::default_mut_book_repository::DefaultMutBookRepository;
use application::repositories::character_repository::default_character_repository::DefaultCharacterRepository;
use application::repositories::character_repository::default_mut_character_repository::DefaultMutCharacterRepository;
use application::repositories::file_repository::default_file_repository::DefaultFileRepository;
use application::repositories::file_repository::default_mut_file_repository::DefaultMutFileRepository;
use application::repositories::franchise_repository::default_franchise_repository::DefaultFranchiseRepository;
use application::repositories::franchise_repository::default_mut_franchise_repository::DefaultMutFranchiseRepository;
use application::repositories::genre_repository::default_genre_repository::DefaultGenreRepository;
use application::repositories::genre_repository::default_mut_genre_repository::DefaultMutGenreRepository;
use application::repositories::image_repository::default_image_repository::DefaultImageRepository;
use application::repositories::image_repository::default_mut_image_repository::DefaultMutImageRepository;
use application::repositories::person_repository::default_mut_person_repository::DefaultMutPersonRepository;
use application::repositories::person_repository::default_person_repository::DefaultPersonRepository;
use application::repositories::role_repository::default_mut_role_repository::DefaultMutRoleRepository;
use application::repositories::role_repository::default_role_repository::DefaultRoleRepository;
use application::repositories::theme_repository::default_mut_theme_repository::DefaultMutThemeRepository;
use application::repositories::theme_repository::default_theme_repository::DefaultThemeRepository;
use application::repositories::user_repository::default_mut_user_repository::DefaultMutUserRepository;
use application::repositories::user_repository::default_user_repository::DefaultUserRepository;
use infrastructure::services::account_service::default_account_service::DefaultAccountService;
use infrastructure::services::account_service::default_mut_account_service::DefaultMutAccountService;
use infrastructure::services::book_service::book_character_service::default_book_character_service::DefaultBookCharacterService;
use infrastructure::services::book_service::book_character_service::default_mut_book_character_service::DefaultMutBookCharacterService;
use infrastructure::services::book_service::book_genre_service::default_book_genre_service::DefaultBookGenreService;
use infrastructure::services::book_service::book_genre_service::default_mut_book_genre_service::DefaultMutBookGenreService;
use infrastructure::services::book_service::book_involved_service::default_book_involved_service::DefaultBookInvolvedService;
use infrastructure::services::book_service::book_involved_service::default_mut_book_involved_service::DefaultMutBookInvolvedService;
use infrastructure::services::book_service::book_theme_service::default_book_theme_service::DefaultBookThemeService;
use infrastructure::services::book_service::book_theme_service::default_mut_book_theme_service::DefaultMutBookThemeService;
use infrastructure::services::book_service::default_book_service::DefaultBookService;
use infrastructure::services::book_service::default_mut_book_service::DefaultMutBookService;
use infrastructure::services::character_service::default_character_service::DefaultCharacterService;
use infrastructure::services::character_service::default_mut_character_service::DefaultMutCharacterService;
use infrastructure::services::file_service::default_file_service::DefaultFileService;
use infrastructure::services::file_service::default_mut_file_service::DefaultMutFileService;
use infrastructure::services::franchise_service::default_franchise_service::DefaultFranchiseService;
use infrastructure::services::franchise_service::default_mut_franchise_service::DefaultMutFranchiseService;
use infrastructure::services::genre_service::default_genre_service::DefaultGenreService;
use infrastructure::services::genre_service::default_mut_genre_service::DefaultMutGenreService;
use infrastructure::services::image_service::default_image_service::DefaultImageService;
use infrastructure::services::image_service::default_mut_image_service::DefaultMutImageService;
use infrastructure::services::person_service::default_mut_person_service::DefaultMutPersonService;
use infrastructure::services::person_service::default_person_service::DefaultPersonService;
use infrastructure::services::role_service::default_mut_role_service::DefaultMutRoleService;
use infrastructure::services::role_service::default_role_service::DefaultRoleService;
use infrastructure::services::theme_service::default_mut_theme_service::DefaultMutThemeService;
use infrastructure::services::theme_service::default_theme_service::DefaultThemeService;
use infrastructure::services::user_service::default_mut_user_service::DefaultMutUserService;
use infrastructure::services::user_service::default_user_service::DefaultUserService;
use repositories::account_repository::mut_account_repository::MutAccountRepository;
use repositories::account_repository::AccountRepository;
use repositories::book_repository::book_character_repository::mut_book_character_repository::MutBookCharacterRepository;
use repositories::book_repository::book_character_repository::BookCharacterRepository;
use repositories::book_repository::book_genre_repository::mut_book_genre_repository::MutBookGenreRepository;
use repositories::book_repository::book_genre_repository::BookGenreRepository;
use repositories::book_repository::book_involved_repository::mut_book_involved_repository::MutBookInvolvedRepository;
use repositories::book_repository::book_involved_repository::BookInvolvedRepository;
use repositories::book_repository::book_theme_repository::mut_book_theme_repository::MutBookThemeRepository;
use repositories::book_repository::book_theme_repository::BookThemeRepository;
use repositories::book_repository::mut_book_repository::MutBookRepository;
use repositories::book_repository::BookRepository;
use repositories::character_repository::mut_character_repository::MutCharacterRepository;
use repositories::character_repository::CharacterRepository;
use repositories::file_repository::mut_file_repository::MutFileRepository;
use repositories::file_repository::FileRepository;
use repositories::franchise_repository::mut_franchise_repository::MutFranchiseRepository;
use repositories::franchise_repository::FranchiseRepository;
use repositories::genre_repository::mut_genre_repository::MutGenreRepository;
use repositories::genre_repository::GenreRepository;
use repositories::image_repository::mut_image_repository::MutImageRepository;
use repositories::image_repository::ImageRepository;
use repositories::person_repository::mut_person_repository::MutPersonRepository;
use repositories::person_repository::PersonRepository;
use repositories::role_repository::mut_role_repository::MutRoleRepository;
use repositories::role_repository::RoleRepository;
use repositories::theme_repository::mut_theme_repository::MutThemeRepository;
use repositories::theme_repository::ThemeRepository;
use repositories::user_repository::mut_user_repository::MutUserRepository;
use repositories::user_repository::UserRepository;
use services::account_service::mut_account_service::MutAccountService;
use services::account_service::AccountService;
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
use services::character_service::mut_character_service::MutCharacterService;
use services::character_service::CharacterService;
use services::file_service::mut_file_service::MutFileService;
use services::file_service::FileService;
use services::franchise_service::mut_franchise_service::MutFranchiseService;
use services::franchise_service::FranchiseService;
use services::genre_service::mut_genre_service::MutGenreService;
use services::genre_service::GenreService;
use services::image_service::mut_image_service::MutImageService;
use services::image_service::ImageService;
use services::person_service::mut_person_service::MutPersonService;
use services::person_service::PersonService;
use services::role_service::mut_role_service::MutRoleService;
use services::role_service::RoleService;
use services::theme_service::mut_theme_service::MutThemeService;
use services::theme_service::ThemeService;
use services::user_service::mut_user_service::MutUserService;
use services::user_service::UserService;

pub fn get_book_service<'a>(book_repository: Arc<dyn BookRepository + 'a>) -> impl BookService + 'a {
  DefaultBookService::new(book_repository)
}

pub fn get_book_repository<'a>(
  client: &'a Client,
  image_repository: Arc<dyn ImageRepository + 'a>,
  franchise_repository: Arc<dyn FranchiseRepository + 'a>,
  book_genre_repository: Arc<dyn BookGenreRepository + 'a>,
  book_theme_repository: Arc<dyn BookThemeRepository + 'a>,
  book_involved_repository: Arc<dyn BookInvolvedRepository + 'a>,
  book_character_repository: Arc<dyn BookCharacterRepository + 'a>,
) -> impl BookRepository + 'a {
  DefaultBookRepository::new(
    client,
    image_repository,
    franchise_repository,
    book_genre_repository,
    book_theme_repository,
    book_involved_repository,
    book_character_repository,
  )
}

pub fn get_image_repository(client: &Client) -> impl ImageRepository + '_ {
  DefaultImageRepository::new(client)
}

pub fn get_franchise_repository(client: &Client) -> impl FranchiseRepository + '_ {
  DefaultFranchiseRepository::new(client)
}

pub fn get_franchise_service<'a>(
  franchise_repository: Arc<dyn FranchiseRepository + 'a>,
) -> impl FranchiseService + 'a {
  DefaultFranchiseService::new(franchise_repository)
}

pub fn get_image_service<'a>(image_repository: Arc<dyn ImageRepository + 'a>) -> impl ImageService + 'a {
  DefaultImageService::new(image_repository)
}

pub fn get_genre_service<'a>(genre_repository: Arc<dyn GenreRepository + 'a>) -> impl GenreService + 'a {
  DefaultGenreService::new(genre_repository)
}

pub fn get_genre_repository(client: &Client) -> impl GenreRepository + '_ {
  DefaultGenreRepository::new(client)
}

pub fn get_theme_service<'a>(theme_repository: Arc<dyn ThemeRepository + 'a>) -> impl ThemeService + 'a {
  DefaultThemeService::new(theme_repository)
}

pub fn get_theme_repository(client: &Client) -> impl ThemeRepository + '_ {
  DefaultThemeRepository::new(client)
}

pub fn get_person_service<'a>(person_repository: Arc<dyn PersonRepository + 'a>) -> impl PersonService + 'a {
  DefaultPersonService::new(person_repository)
}

pub fn get_person_repository<'a>(
  client: &'a Client,
  image_repository: Arc<dyn ImageRepository + 'a>,
) -> impl PersonRepository + 'a {
  DefaultPersonRepository::new(client, image_repository)
}

pub fn get_character_service<'a>(
  character_repository: Arc<dyn CharacterRepository + 'a>,
) -> impl CharacterService + 'a {
  DefaultCharacterService::new(character_repository)
}

pub fn get_character_repository<'a>(
  client: &'a Client,
  image_repository: Arc<dyn ImageRepository + 'a>,
) -> impl CharacterRepository + 'a {
  DefaultCharacterRepository::new(client, image_repository)
}

pub fn get_book_genre_service<'a>(
  book_genre_repository: Arc<dyn BookGenreRepository + 'a>,
) -> impl BookGenreService + 'a {
  DefaultBookGenreService::new(book_genre_repository)
}

pub fn get_book_genre_repository<'a>(
  client: &'a Client,
  genre_repository: Arc<dyn GenreRepository + 'a>,
) -> impl BookGenreRepository + 'a {
  DefaultBookGenreRepository::new(client, genre_repository)
}

pub fn get_book_theme_service<'a>(
  book_theme_repository: Arc<dyn BookThemeRepository + 'a>,
) -> impl BookThemeService + 'a {
  DefaultBookThemeService::new(book_theme_repository)
}

pub fn get_book_theme_repository<'a>(
  client: &'a Client,
  theme_repository: Arc<dyn ThemeRepository + 'a>,
) -> impl BookThemeRepository + 'a {
  DefaultBookThemeRepository::new(client, theme_repository)
}

pub fn get_book_character_service<'a>(
  book_character_repository: Arc<dyn BookCharacterRepository + 'a>,
) -> impl BookCharacterService + 'a {
  DefaultBookCharacterService::new(book_character_repository)
}

pub fn get_book_character_repository<'a>(
  client: &'a Client,
  character_repository: Arc<dyn CharacterRepository + 'a>,
) -> impl BookCharacterRepository + 'a {
  DefaultBookCharacterRepository::new(client, character_repository)
}

pub fn get_mut_book_theme_service<'a>(
  book_repository: Arc<dyn BookRepository + 'a>,
  book_theme_repository: Arc<dyn BookThemeRepository + 'a>,
  mut_book_theme_repository: Arc<dyn MutBookThemeRepository + 'a>,
  theme_repository: Arc<dyn ThemeRepository + 'a>,
) -> impl MutBookThemeService + 'a {
  DefaultMutBookThemeService::new(
    book_repository,
    book_theme_repository,
    mut_book_theme_repository,
    theme_repository,
  )
}

pub fn get_mut_book_theme_repository<'a>(transaction: &'a Transaction) -> impl MutBookThemeRepository + 'a {
  DefaultMutBookThemeRepository::new(transaction)
}

pub fn get_mut_book_genre_service<'a>(
  book_repository: Arc<dyn BookRepository + 'a>,
  book_genre_repository: Arc<dyn BookGenreRepository + 'a>,
  mut_book_genre_repository: Arc<dyn MutBookGenreRepository + 'a>,
  genre_repository: Arc<dyn GenreRepository + 'a>,
) -> impl MutBookGenreService + 'a {
  DefaultMutBookGenreService::new(
    book_repository,
    book_genre_repository,
    mut_book_genre_repository,
    genre_repository,
  )
}

pub fn get_mut_book_genre_repository<'a>(transaction: &'a Transaction) -> impl MutBookGenreRepository + 'a {
  DefaultMutBookGenreRepository::new(transaction)
}

pub fn get_mut_book_character_service<'a>(
  book_repository: Arc<dyn BookRepository + 'a>,
  book_character_repository: Arc<dyn BookCharacterRepository + 'a>,
  mut_book_character_repository: Arc<dyn MutBookCharacterRepository + 'a>,
  character_repository: Arc<dyn CharacterRepository + 'a>,
) -> impl MutBookCharacterService + 'a {
  DefaultMutBookCharacterService::new(
    book_repository,
    book_character_repository,
    mut_book_character_repository,
    character_repository,
  )
}

pub fn get_mut_book_character_repository<'a>(transaction: &'a Transaction) -> impl MutBookCharacterRepository + 'a {
  DefaultMutBookCharacterRepository::new(transaction)
}

pub fn get_book_involved_service<'a>(
  book_involved_repository: Arc<dyn BookInvolvedRepository + 'a>,
) -> impl BookInvolvedService + 'a {
  DefaultBookInvolvedService::new(book_involved_repository)
}

pub fn get_book_involved_repository<'a>(
  client: &'a Client,
  person_repository: Arc<dyn PersonRepository + 'a>,
  role_repository: Arc<dyn RoleRepository + 'a>,
) -> impl BookInvolvedRepository + 'a {
  DefaultBookInvolvedRepository::new(client, person_repository, role_repository)
}

pub fn get_role_service<'a>(role_repository: Arc<dyn RoleRepository + 'a>) -> impl RoleService + 'a {
  DefaultRoleService::new(role_repository)
}

pub fn get_role_repository(client: &Client) -> impl RoleRepository + '_ {
  DefaultRoleRepository::new(client)
}

pub fn get_file_service<'a>(file_repository: Arc<dyn FileRepository + 'a>) -> impl FileService + 'a {
  DefaultFileService::new(file_repository)
}

pub fn get_file_repository<'a>() -> impl FileRepository + 'a {
  DefaultFileRepository::new()
}

pub fn get_mut_file_service<'a>(mut_file_repository: Arc<dyn MutFileRepository + 'a>) -> impl MutFileService + 'a {
  DefaultMutFileService::new(mut_file_repository)
}

pub fn get_mut_file_repository<'a>() -> impl MutFileRepository + 'a {
  DefaultMutFileRepository::new()
}

pub fn get_mut_image_service<'a>(
  mut_image_repository: Arc<dyn MutImageRepository + 'a>,
  mut_file_service: Arc<dyn MutFileService + 'a>,
  display_path: &'a str,
  path: &'a str,
) -> impl MutImageService + 'a {
  DefaultMutImageService::new(mut_image_repository, mut_file_service, display_path, path)
}

pub fn get_mut_image_repository<'a>(
  transaction: &'a Transaction,
  image_repository: Arc<dyn ImageRepository + 'a>,
  mut_file_repository: Arc<dyn MutFileRepository + 'a>,
  file_repository: Arc<dyn FileRepository + 'a>,
) -> impl MutImageRepository + 'a {
  DefaultMutImageRepository::new(transaction, image_repository, mut_file_repository, file_repository)
}

pub fn get_user_service<'a>(user_repository: Arc<dyn UserRepository + 'a>) -> impl UserService + 'a {
  DefaultUserService::new(user_repository)
}

pub fn get_user_repository<'a>(
  connection: &'a Client,
  image_repository: Arc<dyn ImageRepository + 'a>,
) -> impl UserRepository + 'a {
  DefaultUserRepository::new(connection, image_repository)
}

pub fn get_mut_user_service<'a>(
  mut_user_repository: Arc<dyn MutUserRepository + 'a>,
  mut_image_service: Arc<dyn MutImageService + 'a>,
) -> impl MutUserService + 'a {
  DefaultMutUserService::new(mut_user_repository, mut_image_service)
}

pub fn get_mut_user_repository<'a>(
  transaction: &'a Transaction,
  user_repository: Arc<dyn UserRepository + 'a>,
) -> impl MutUserRepository + 'a {
  DefaultMutUserRepository::new(transaction, user_repository)
}

pub fn get_mut_account_repository<'a>(
  transaction: &'a Transaction,
  account_repository: Arc<dyn AccountRepository + 'a>,
) -> impl MutAccountRepository + 'a {
  DefaultMutAccountRepository::new(transaction, account_repository)
}

pub fn get_mut_account_service<'a>(
  mut_account_repository: Arc<dyn MutAccountRepository + 'a>,
  account_service: Arc<dyn AccountService + 'a>,
  mut_user_service: Arc<dyn MutUserService + 'a>,
) -> impl MutAccountService + 'a {
  DefaultMutAccountService::new(mut_account_repository, account_service, mut_user_service)
}

pub fn get_account_service<'a>(account_repository: Arc<dyn AccountRepository + 'a>) -> impl AccountService + 'a {
  DefaultAccountService::new(account_repository)
}

pub fn get_account_repository<'a>(
  connection: &'a Client,
  user_repository: Arc<dyn UserRepository + 'a>,
) -> impl AccountRepository + 'a {
  DefaultAccountRepository::new(connection, user_repository)
}

pub fn get_mut_book_involved_service<'a>(
  book_repository: Arc<dyn BookRepository + 'a>,
  book_involved_repository: Arc<dyn BookInvolvedRepository + 'a>,
  mut_book_involved_repository: Arc<dyn MutBookInvolvedRepository + 'a>,
  person_repository: Arc<dyn PersonRepository + 'a>,
  role_repository: Arc<dyn RoleRepository + 'a>,
) -> impl MutBookInvolvedService + 'a {
  DefaultMutBookInvolvedService::new(
    book_repository,
    book_involved_repository,
    mut_book_involved_repository,
    person_repository,
    role_repository,
  )
}

pub fn get_mut_book_involved_repository<'a>(transaction: &'a Transaction) -> impl MutBookInvolvedRepository + 'a {
  DefaultMutBookInvolvedRepository::new(transaction)
}

pub fn get_mut_book_service<'a>(
  book_repository: Arc<dyn BookRepository + 'a>,
  mut_book_repository: Arc<dyn MutBookRepository + 'a>,
  mut_image_service: Arc<dyn MutImageService + 'a>,
  franchise_repository: Arc<dyn FranchiseRepository + 'a>,
  theme_repository: Arc<dyn ThemeRepository + 'a>,
  genre_repository: Arc<dyn GenreRepository + 'a>,
  character_repository: Arc<dyn CharacterRepository + 'a>,
  person_repository: Arc<dyn PersonRepository + 'a>,
  role_repository: Arc<dyn RoleRepository + 'a>,
) -> impl MutBookService + 'a {
  DefaultMutBookService::new(
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

pub fn get_mut_book_repository<'a>(
  transaction: &'a Transaction<'a>,
  mut_book_genre_repository: Arc<dyn MutBookGenreRepository + 'a>,
  mut_book_character_repository: Arc<dyn MutBookCharacterRepository + 'a>,
  mut_book_theme_repository: Arc<dyn MutBookThemeRepository + 'a>,
  mut_book_involved_repository: Arc<dyn MutBookInvolvedRepository + 'a>,
  book_repository: Arc<dyn BookRepository + 'a>,
) -> impl MutBookRepository + 'a {
  DefaultMutBookRepository::new(
    transaction,
    mut_book_genre_repository,
    mut_book_character_repository,
    mut_book_theme_repository,
    mut_book_involved_repository,
    book_repository,
  )
}

pub fn get_mut_person_service<'a>(
  person_repository: Arc<dyn PersonRepository + 'a>,
  mut_person_repository: Arc<dyn MutPersonRepository + 'a>,
  mut_image_service: Arc<dyn MutImageService + 'a>,
) -> impl MutPersonService + 'a {
  DefaultMutPersonService::new(person_repository, mut_person_repository, mut_image_service)
}

pub fn get_mut_person_repository<'a>(
  transaction: &'a Transaction<'a>,
  person_repository: Arc<dyn PersonRepository + 'a>,
) -> impl MutPersonRepository + 'a {
  DefaultMutPersonRepository::new(transaction, person_repository)
}

pub fn get_mut_genre_service<'a>(
  genre_repository: Arc<dyn GenreRepository + 'a>,
  mut_genre_repository: Arc<dyn MutGenreRepository + 'a>,
) -> impl MutGenreService + 'a {
  DefaultMutGenreService::new(genre_repository, mut_genre_repository)
}

pub fn get_mut_genre_repository<'a>(
  transaction: &'a Transaction<'a>,
  genre_repository: Arc<dyn GenreRepository + 'a>,
) -> impl MutGenreRepository + 'a {
  DefaultMutGenreRepository::new(transaction, genre_repository)
}

pub fn get_mut_theme_service<'a>(
  theme_repository: Arc<dyn ThemeRepository + 'a>,
  mut_theme_repository: Arc<dyn MutThemeRepository + 'a>,
) -> impl MutThemeService + 'a {
  DefaultMutThemeService::new(theme_repository, mut_theme_repository)
}

pub fn get_mut_theme_repository<'a>(
  transaction: &'a Transaction<'a>,
  theme_repository: Arc<dyn ThemeRepository + 'a>,
) -> impl MutThemeRepository + 'a {
  DefaultMutThemeRepository::new(transaction, theme_repository)
}

pub fn get_mut_role_service<'a>(
  role_repository: Arc<dyn RoleRepository + 'a>,
  mut_role_repository: Arc<dyn MutRoleRepository + 'a>,
) -> impl MutRoleService + 'a {
  DefaultMutRoleService::new(role_repository, mut_role_repository)
}

pub fn get_mut_role_repository<'a>(
  transaction: &'a Transaction<'a>,
  role_repository: Arc<dyn RoleRepository + 'a>,
) -> impl MutRoleRepository + 'a {
  DefaultMutRoleRepository::new(transaction, role_repository)
}
pub fn get_mut_franchise_service<'a>(
  franchise_repository: Arc<dyn FranchiseRepository + 'a>,
  mut_franchise_repository: Arc<dyn MutFranchiseRepository + 'a>,
) -> impl MutFranchiseService + 'a {
  DefaultMutFranchiseService::new(franchise_repository, mut_franchise_repository)
}

pub fn get_mut_franchise_repository<'a>(
  transaction: &'a Transaction<'a>,
  franchise_repository: Arc<dyn FranchiseRepository + 'a>,
) -> impl MutFranchiseRepository + 'a {
  DefaultMutFranchiseRepository::new(transaction, franchise_repository)
}
pub fn get_mut_character_service<'a>(
  character_repository: Arc<dyn CharacterRepository + 'a>,
  mut_character_repository: Arc<dyn MutCharacterRepository + 'a>,
  mut_image_service: Arc<dyn MutImageService + 'a>,
) -> impl MutCharacterService + 'a {
  DefaultMutCharacterService::new(character_repository, mut_character_repository, mut_image_service)
}

pub fn get_mut_character_repository<'a>(
  transaction: &'a Transaction<'a>,
  character_repository: Arc<dyn CharacterRepository + 'a>,
) -> impl MutCharacterRepository + 'a {
  DefaultMutCharacterRepository::new(transaction, character_repository)
}
