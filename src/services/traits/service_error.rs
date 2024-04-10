use std::error::Error;

pub enum ServiceError {
  ClientError(Box<dyn Error>),
  ServerError(Box<dyn Error>),
}
