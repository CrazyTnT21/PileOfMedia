use std::error::Error;

#[derive(Debug)]
pub enum ServiceError {
  ClientError(Box<dyn Error>),
  ServerError(Box<dyn Error>),
}
