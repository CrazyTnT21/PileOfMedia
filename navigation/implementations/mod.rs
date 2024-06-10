use std::sync::Arc;
use tokio_postgres::{Client, Transaction};
use application::repositories::account_repository::default_account_repository::DefaultAccountRepository;
use application::repositories::account_repository::default_mut_account_repository::DefaultMutAccountRepository;

use application::repositories::default_book_relations_repository::DefaultBookRelationsRepository;
use application::repositories::default_book_repository::DefaultBookRepository;
use application::repositories::default_character_repository::DefaultCharacterRepository;
use application::repositories::default_genre_repository::DefaultGenreRepository;
use application::repositories::default_person_repository::DefaultPersonRepository;
use application::repositories::default_role_repository::DefaultRoleRepository;
use application::repositories::default_theme_repository::DefaultThemeRepository;
use application::repositories::file_repository::default_file_repository::DefaultFileRepository;
use application::repositories::file_repository::default_mut_file_repository::DefaultMutFileRepository;
use application::repositories::image_repository::default_image_repository::DefaultImageRepository;
use application::repositories::image_repository::default_mut_image_repository::DefaultMutImageRepository;
use application::repositories::user_repository::default_mut_user_repository::DefaultMutUserRepository;
use application::repositories::user_repository::default_user_repository::DefaultUserRepository;
use domain::enums::language::Language;
use infrastructure::services::account_service::default_account_service::DefaultAccountService;
use infrastructure::services::account_service::default_mut_account_service::DefaultMutAccountService;
use infrastructure::services::default_book_relations_service::DefaultBookRelationsService;
use infrastructure::services::default_book_service::DefaultBookService;
use infrastructure::services::default_character_service::DefaultCharacterService;
use infrastructure::services::default_genre_service::DefaultGenreService;
use infrastructure::services::default_person_service::DefaultPersonService;
use infrastructure::services::default_role_service::DefaultRoleService;
use infrastructure::services::default_theme_service::DefaultThemeService;
use infrastructure::services::file_service::default_file_service::DefaultFileService;
use infrastructure::services::file_service::default_mut_file_service::DefaultMutFileService;
use infrastructure::services::image_service::default_image_service::DefaultImageService;
use infrastructure::services::image_service::default_mut_image_service::DefaultMutImageService;
use infrastructure::services::user_service::default_mut_user_service::DefaultMutUserService;
use infrastructure::services::user_service::default_user_service::DefaultUserService;
use repositories::account_repository::AccountRepository;
use repositories::account_repository::mut_account_repository::MutAccountRepository;
use repositories::book_relations_repository::BookRelationsRepository;
use repositories::book_repository::BookRepository;
use repositories::character_repository::CharacterRepository;
use repositories::file_repository::FileRepository;
use repositories::file_repository::mut_file_repository::MutFileRepository;
use repositories::genre_repository::GenreRepository;
use repositories::image_repository::ImageRepository;
use repositories::image_repository::mut_image_repository::MutImageRepository;
use repositories::person_repository::PersonRepository;
use repositories::role_repository::RoleRepository;
use repositories::theme_repository::ThemeRepository;
use repositories::user_repository::mut_user_repository::MutUserRepository;
use repositories::user_repository::UserRepository;
use services::account_service::AccountService;
use services::account_service::mut_account_service::MutAccountService;
use services::book_relations_service::BookRelationsService;
use services::book_service::BookService;
use services::character_service::CharacterService;
use services::file_service::FileService;
use services::file_service::mut_file_service::MutFileService;
use services::genre_service::GenreService;
use services::image_service::ImageService;
use services::image_service::mut_image_service::MutImageService;
use services::person_service::PersonService;
use services::role_service::RoleService;
use services::theme_service::ThemeService;
use services::user_service::mut_user_service::MutUserService;
use services::user_service::UserService;

pub fn get_book_service<'a>(book_repository: Arc<dyn BookRepository + 'a>) -> impl BookService + 'a {
  DefaultBookService::new(book_repository)
}

pub fn get_book_repository<'a>(client: &'a Client, language: Language, image_repository: Arc<dyn ImageRepository + 'a>) -> impl BookRepository + 'a {
  DefaultBookRepository::new(client, language, image_repository)
}

pub fn get_image_repository(client: &Client) -> impl ImageRepository + '_ {
  DefaultImageRepository::new(client)
}

pub fn get_image_service<'a>(image_repository: Arc<dyn ImageRepository + 'a>) -> impl ImageService + 'a {
  DefaultImageService::new(image_repository)
}

pub fn get_genre_service<'a>(genre_repository: Arc<dyn GenreRepository + 'a>) -> impl GenreService + 'a {
  DefaultGenreService::new(genre_repository)
}

pub fn get_genre_repository(client: &Client, language: Language) -> impl GenreRepository + '_ {
  DefaultGenreRepository::new(client, language)
}

pub fn get_theme_service<'a>(theme_repository: Arc<dyn ThemeRepository + 'a>) -> impl ThemeService + 'a {
  DefaultThemeService::new(theme_repository)
}

pub fn get_theme_repository(client: &Client, language: Language) -> impl ThemeRepository + '_ {
  DefaultThemeRepository::new(client, language)
}

pub fn get_person_service<'a>(person_repository: Arc<dyn PersonRepository + 'a>) -> impl PersonService + 'a {
  DefaultPersonService::new(person_repository)
}

pub fn get_person_repository<'a>(client: &'a Client, language: Language, image_repository: Arc<dyn ImageRepository + 'a>) -> impl PersonRepository + 'a {
  DefaultPersonRepository::new(client, language, image_repository)
}

pub fn get_character_service<'a>(character_repository: Arc<dyn CharacterRepository + 'a>) -> impl CharacterService + 'a {
  DefaultCharacterService::new(character_repository)
}

pub fn get_character_repository<'a>(client: &'a Client, language: Language, image_repository: Arc<dyn ImageRepository + 'a>) -> impl CharacterRepository + 'a {
  DefaultCharacterRepository::new(client, language, image_repository)
}

pub fn get_book_relations_service<'a>(book_relations_repository: Arc<dyn BookRelationsRepository + 'a>) -> impl BookRelationsService + 'a {
  DefaultBookRelationsService::new(book_relations_repository)
}

pub fn get_book_relations_repository<'a>(client: &'a Client,
                                         language: Language,
                                         book_repository: Arc<dyn BookRepository + 'a>,
                                         genre_repository: Arc<dyn GenreRepository + 'a>,
                                         theme_repository: Arc<dyn ThemeRepository + 'a>,
                                         character_repository: Arc<dyn CharacterRepository + 'a>,
                                         person_repository: Arc<dyn PersonRepository + 'a>,
                                         role_repository: Arc<dyn RoleRepository + 'a>, ) -> impl BookRelationsRepository + 'a
{
  DefaultBookRelationsRepository::new(client, language, book_repository, genre_repository, theme_repository, character_repository, person_repository, role_repository)
}

pub fn get_role_service<'a>(role_repository: Arc<dyn RoleRepository + 'a>) -> impl RoleService + 'a {
  DefaultRoleService::new(role_repository)
}

pub fn get_role_repository(client: &Client, language: Language) -> impl RoleRepository + '_ {
  DefaultRoleRepository::new(client, language)
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

pub fn get_mut_image_service<'a>(mut_image_repository: Arc<dyn MutImageRepository + 'a>, mut_file_service: Arc<dyn MutFileService + 'a>, display_path: &'a str, path: &'a str) -> impl MutImageService + 'a {
  DefaultMutImageService::new(mut_image_repository, mut_file_service, display_path, path)
}

pub fn get_mut_image_repository<'a>(transaction: &'a Transaction, image_repository: Arc<dyn ImageRepository + 'a>, mut_file_repository: Arc<dyn MutFileRepository + 'a>, file_repository: Arc<dyn FileRepository + 'a>) -> impl MutImageRepository + 'a {
  DefaultMutImageRepository::new(transaction, image_repository, mut_file_repository, file_repository)
}

pub fn get_user_service<'a>(user_repository: Arc<dyn UserRepository + 'a>) -> impl UserService + 'a {
  DefaultUserService::new(user_repository)
}

pub fn get_user_repository<'a>(connection: &'a Client, image_repository: Arc<dyn ImageRepository + 'a>) -> impl UserRepository + 'a {
  DefaultUserRepository::new(connection, image_repository)
}

pub fn get_mut_user_service<'a>(mut_user_repository: Arc<dyn MutUserRepository + 'a>, mut_image_service: Arc<dyn MutImageService + 'a>) -> impl MutUserService + 'a {
  DefaultMutUserService::new(mut_user_repository, mut_image_service)
}

pub fn get_mut_user_repository<'a>(transaction: &'a Transaction, user_repository: Arc<dyn UserRepository + 'a>, image_repository: Arc<dyn ImageRepository + 'a>) -> impl MutUserRepository + 'a {
  DefaultMutUserRepository::new(transaction, user_repository, image_repository)
}

pub fn get_mut_account_repository<'a>(transaction: &'a Transaction, account_repository: Arc<dyn AccountRepository + 'a>, user_repository: Arc<dyn UserRepository + 'a>) -> impl MutAccountRepository + 'a {
  DefaultMutAccountRepository::new(transaction, account_repository, user_repository)
}

pub fn get_mut_account_service<'a>(mut_account_repository: Arc<dyn MutAccountRepository + 'a>, account_service: Arc<dyn AccountService + 'a>, mut_user_service: Arc<dyn MutUserService + 'a>) -> impl MutAccountService + 'a {
  DefaultMutAccountService::new(mut_account_repository, account_service, mut_user_service)
}

pub fn get_account_service<'a>(account_repository: Arc<dyn AccountRepository + 'a>) -> impl AccountService + 'a {
  DefaultAccountService::new(account_repository)
}

pub fn get_account_repository<'a>(connection: &'a Client, user_repository: Arc<dyn UserRepository + 'a>) -> impl AccountRepository + 'a {
  DefaultAccountRepository::new(connection, user_repository)
}
