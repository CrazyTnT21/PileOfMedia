use std::error::Error;
use services::traits::service_error::ServiceError;

pub mod default_book_service;
pub mod default_image_service;
pub mod default_file_service;
pub mod default_genre_service;
pub mod default_theme_service;
pub mod default_person_service;
pub mod default_character_service;
pub mod default_role_service;
pub mod default_book_relations_service;

fn map_server_error(error: Box<dyn Error>) -> ServiceError {
  ServiceError::ServerError(error)
}
