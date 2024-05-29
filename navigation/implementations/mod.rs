use application::Pooled;
use application::repositories::default_book_repository::DefaultBookRepository;
use application::repositories::default_image_repository::DefaultImageRepository;
use infrastructure::services::default_book_service::DefaultBookService;
use infrastructure::services::default_image_service::DefaultImageService;
use repositories::book_repository::BookRepository;
use repositories::image_repository::ImageRepository;
use services::book_service::BookService;
use services::image_service::ImageService;
use crate::controllers::DEFAULT_LANGUAGE;

pub fn get_book_service(book_repository: &impl BookRepository) -> impl BookService + '_ {
  DefaultBookService::new(book_repository)
}

pub fn get_book_repository<'a>(pool: &'a Pooled, image_repository: &'a impl ImageRepository) -> impl BookRepository + 'a {
  DefaultBookRepository::new(pool, DEFAULT_LANGUAGE, image_repository)
}

pub fn get_image_repository<'a>(pool: &'a Pooled) -> impl ImageRepository + 'a {
  DefaultImageRepository::new(pool)
}

pub fn get_image_service<'a>(image_repository: &'a impl ImageRepository) -> impl ImageService + 'a {
  DefaultImageService::new(image_repository)
}
