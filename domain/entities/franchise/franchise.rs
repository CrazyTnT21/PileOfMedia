use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct Franchise {
  pub id: i32,
  pub name: String,
}
