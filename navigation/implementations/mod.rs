use application::Pooled;
use application::repositories::default_book_repository::DefaultBookRepository;
use application::repositories::default_genre_repository::DefaultGenreRepository;
use application::repositories::default_image_repository::DefaultImageRepository;
use domain::enums::language::Language;
use infrastructure::services::default_book_service::DefaultBookService;
use infrastructure::services::default_genre_service::DefaultGenreService;
use infrastructure::services::default_image_service::DefaultImageService;
use repositories::book_repository::BookRepository;
use repositories::genre_repository::GenreRepository;
use repositories::image_repository::ImageRepository;
use services::book_service::BookService;
use services::genre_service::GenreService;
use services::image_service::ImageService;

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
