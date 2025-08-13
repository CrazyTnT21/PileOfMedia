use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
pub enum ServiceError<T> {
  ClientError(T),
  ServerError(Box<dyn Error>),
}

impl<T: Display> Display for ServiceError<T> {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}",
      match self {
        ServiceError::ClientError(client_error) => client_error.to_string(),
        ServiceError::ServerError(_) => "Internal server error".to_string(),
      }
    )
  }
}

impl<T> From<Box<dyn Error>> for ServiceError<T> {
  fn from(value: Box<dyn Error>) -> Self {
    ServiceError::ServerError(value)
  }
}
impl<T> ServiceError<T> {
  pub fn server_error(value: Box<dyn Error>) -> Self {
    Self::ServerError(value)
  }
}

pub trait ServiceErrorMap<T, E> {
  fn map_service_error(self) -> Result<T, ServiceError<E>>;
}
impl<T: Error + 'static> Error for ServiceError<T> {
  fn source(&self) -> Option<&(dyn Error + 'static)> {
    match self {
      ServiceError::ClientError(x) => Some(x),
      ServiceError::ServerError(x) => Some(&**x),
    }
  }
}
