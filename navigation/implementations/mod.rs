use tokio_postgres::{Client, Transaction};

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
use domain::enums::language::Language;
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
use services::book_relations_service::BookRelationsService;
use services::character_service::CharacterService;
use services::file_service::FileService;
use services::file_service::mut_file_service::MutFileService;
use services::genre_service::GenreService;
use services::image_service::ImageService;
use services::image_service::mut_image_service::MutImageService;
use services::person_service::PersonService;
use services::role_service::RoleService;
use services::theme_service::ThemeService;

pub fn get_book_service(book_repository: &impl BookRepository) -> DefaultBookService {
  DefaultBookService::new(book_repository)
}

pub fn get_book_repository<'a>(client: &'a Client, language: Language, image_repository: &'a impl ImageRepository) -> impl BookRepository + 'a {
  DefaultBookRepository::new(client, language, image_repository)
}

pub fn get_image_repository(client: &Client) -> impl ImageRepository + '_ {
  DefaultImageRepository::new(client)
}

pub fn get_image_service(image_repository: &impl ImageRepository) -> impl ImageService + '_ {
  DefaultImageService::new(image_repository)
}

pub fn get_genre_service(genre_repository: &impl GenreRepository) -> impl GenreService + '_ {
  DefaultGenreService::new(genre_repository)
}

pub fn get_genre_repository(client: &Client, language: Language) -> impl GenreRepository + '_ {
  DefaultGenreRepository::new(client, language)
}

pub fn get_theme_service(theme_repository: &impl ThemeRepository) -> impl ThemeService + '_ {
  DefaultThemeService::new(theme_repository)
}

pub fn get_theme_repository(client: &Client, language: Language) -> impl ThemeRepository + '_ {
  DefaultThemeRepository::new(client, language)
}

pub fn get_person_service(person_repository: &impl PersonRepository) -> impl PersonService + '_ {
  DefaultPersonService::new(person_repository)
}

pub fn get_person_repository<'a>(client: &'a Client, language: Language, image_repository: &'a impl ImageRepository) -> impl PersonRepository + 'a {
  DefaultPersonRepository::new(client, language, image_repository)
}

pub fn get_character_service(character_repository: &impl CharacterRepository) -> impl CharacterService + '_ {
  DefaultCharacterService::new(character_repository)
}

pub fn get_character_repository<'a>(client: &'a Client, language: Language, image_repository: &'a impl ImageRepository) -> impl CharacterRepository + 'a {
  DefaultCharacterRepository::new(client, language, image_repository)
}

pub fn get_book_relations_service(book_relations_repository: &impl BookRelationsRepository) -> impl BookRelationsService + '_ {
  DefaultBookRelationsService::new(book_relations_repository)
}

pub fn get_book_relations_repository<'a>(client: &'a Client,
                                         language: Language,
                                         book_repository: &'a impl BookRepository,
                                         genre_repository: &'a impl GenreRepository,
                                         theme_repository: &'a impl ThemeRepository,
                                         character_repository: &'a impl CharacterRepository,
                                         person_repository: &'a impl PersonRepository,
                                         role_repository: &'a dyn RoleRepository, ) -> impl BookRelationsRepository + 'a
{
  DefaultBookRelationsRepository::new(client, language, book_repository, genre_repository, theme_repository, character_repository, person_repository, role_repository)
}

pub fn get_role_service(role_repository: &impl RoleRepository) -> impl RoleService + '_ {
  DefaultRoleService::new(role_repository)
}

pub fn get_role_repository(client: &Client, language: Language) -> impl RoleRepository + '_ {
  DefaultRoleRepository::new(client, language)
}

pub fn get_file_service(file_repository: &impl FileRepository) -> impl FileService + '_ {
  DefaultFileService::new(file_repository)
}

pub fn get_file_repository<'a>() -> impl FileRepository + 'a {
  DefaultFileRepository::new()
}

pub fn get_mut_file_service(mut_file_repository: &impl MutFileRepository) -> impl MutFileService + '_ {
  DefaultMutFileService::new(mut_file_repository)
}

pub fn get_mut_file_repository<'a>() -> impl MutFileRepository + 'a {
  DefaultMutFileRepository::new()
}

pub fn get_mut_image_service<'a>(mut_image_repository: &'a impl MutImageRepository, mut_file_service: &'a impl MutFileService, display_path: &'a str, path: &'a str) -> impl MutImageService + 'a {
  DefaultMutImageService::new(mut_image_repository, mut_file_service, display_path,path)
}

pub fn get_mut_image_repository<'a>(transaction: &'a Transaction<'a>, image_repository: &'a impl ImageRepository, mut_file_repository: &'a impl MutFileRepository, file_repository: &'a impl FileRepository) -> impl MutImageRepository + 'a {
  DefaultMutImageRepository::new(transaction, image_repository, mut_file_repository, file_repository)
}
