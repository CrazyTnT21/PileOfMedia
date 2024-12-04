use std::error::Error;
use std::fmt::{Display, Formatter};

pub trait Single<T> {
  type Error;
  fn single(self) -> Result<T, Self::Error>;
}
impl<T> Single<T> for Vec<T> {
  type Error = SingleVecError;

  fn single(mut self) -> Result<T, Self::Error> {
    if self.is_empty() {
      return Err(SingleVecError::NoItems);
    };
    if self.len() > 1 {
      return Err(SingleVecError::MoreThanOneItem(self.len()));
    }
    Ok(self.swap_remove(0))
  }
}
#[derive(Debug)]
pub enum SingleVecError {
  NoItems,
  MoreThanOneItem(usize),
}
impl Display for SingleVecError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}",
      match self {
        SingleVecError::NoItems => "The vec contains no items".to_string(),
        SingleVecError::MoreThanOneItem(value) => format!("The vec contains more than one item: {value}"),
      }
    )
  }
}

impl Error for SingleVecError {}
