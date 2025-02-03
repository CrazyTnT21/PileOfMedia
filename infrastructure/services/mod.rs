use std::error::Error;

use services::traits::service_error::ServiceError;

pub mod account_service;
pub mod book_service;
pub mod character_service;
pub mod file_service;
pub mod franchise_service;
pub mod genre_service;
pub mod image_service;
pub mod person_service;
pub mod role_service;
pub mod theme_service;
pub mod user_service;

fn map_server_error<T>(error: Box<dyn Error>) -> ServiceError<T> {
  ServiceError::ServerError(error)
}
