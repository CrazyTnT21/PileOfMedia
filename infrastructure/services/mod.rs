use std::error::Error;
use services::traits::service_error::ServiceError;

pub mod default_book_service;
pub mod default_image_service;
pub mod default_file_service;
pub mod default_genre_service;

fn map_server_error(error: Box<dyn Error>) -> ServiceError {
  ServiceError::ServerError(error)
}
