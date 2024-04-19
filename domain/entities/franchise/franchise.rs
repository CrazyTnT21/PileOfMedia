use serde::Serialize;

#[derive(Serialize)]
pub struct Franchise {
  pub id: i32,
  pub name: String,
}
