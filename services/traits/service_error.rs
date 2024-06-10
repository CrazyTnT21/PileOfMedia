use std::error::Error;

#[derive(Debug)]
pub enum ServiceError {
  ClientError(ClientError),
  ServerError(Box<dyn Error>),
}

#[derive(Debug)]
pub struct ClientError {
  pub title: String,
  pub description: Option<String>,
}
