use application::Pooled;
use application::repositories::default_book_repository::DefaultBookRepository;
use application::repositories::default_genre_repository::DefaultGenreRepository;
use application::repositories::default_image_repository::DefaultImageRepository;
use application::repositories::default_person_repository::DefaultPersonRepository;
use application::repositories::default_theme_repository::DefaultThemeRepository;
use domain::enums::language::Language;
use infrastructure::services::default_book_service::DefaultBookService;
use infrastructure::services::default_genre_service::DefaultGenreService;
use infrastructure::services::default_image_service::DefaultImageService;
use infrastructure::services::default_person_service::DefaultPersonService;
use infrastructure::services::default_theme_service::DefaultThemeService;
use repositories::book_repository::BookRepository;
use repositories::genre_repository::GenreRepository;
use repositories::image_repository::ImageRepository;
use repositories::person_repository::PersonRepository;
use repositories::theme_repository::ThemeRepository;
use services::book_service::BookService;
use services::genre_service::GenreService;
use services::image_service::ImageService;
use services::person_service::PersonService;
use services::theme_service::ThemeService;

pub fn get_book_service(book_repository: &impl BookRepository) -> impl BookService + '_ {
  DefaultBookService::new(book_repository)
}

pub fn get_book_repository<'a>(pool: &'a Pooled, language: Language, image_repository: &'a impl ImageRepository) -> impl BookRepository + 'a {
  DefaultBookRepository::new(pool, language, image_repository)
}

pub fn get_image_repository<'a>(pool: &'a Pooled) -> impl ImageRepository + 'a {
  DefaultImageRepository::new(pool)
}

pub fn get_image_service(image_repository: &impl ImageRepository) -> impl ImageService + '_ {
  DefaultImageService::new(image_repository)
}

pub fn get_genre_service(genre_repository: &impl GenreRepository) -> impl GenreService + '_ {
  DefaultGenreService::new(genre_repository)
}

pub fn get_genre_repository<'a>(pool: &'a Pooled, language: Language) -> impl GenreRepository + 'a {
  DefaultGenreRepository::new(pool, language)
}

pub fn get_theme_service(theme_repository: &impl ThemeRepository) -> impl ThemeService + '_ {
  DefaultThemeService::new(theme_repository)
}

pub fn get_theme_repository<'a>(pool: &'a Pooled, language: Language) -> impl ThemeRepository + 'a {
  DefaultThemeRepository::new(pool, language)
}

pub fn get_person_service(person_repository: &impl PersonRepository) -> impl PersonService + '_ {
  DefaultPersonService::new(person_repository)
}

pub fn get_person_repository<'a>(pool: &'a Pooled, language: Language, image_repository: &'a impl ImageRepository) -> impl PersonRepository + 'a {
  DefaultPersonRepository::new(pool, language, image_repository)
}
