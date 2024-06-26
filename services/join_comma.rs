use std::fmt::Display;

pub trait JoinComma<T> {
  fn join_comma(&self) -> String;
}

impl<T: Display> JoinComma<T> for Vec<T> {
  fn join_comma(&self) -> String {
    self.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(",")
  }
}
