pub struct Pagination {
  pub page: u32,
  pub count: u32,
}

impl Default for Pagination {
  fn default() -> Self {
    Pagination { page: 0, count: 50 }
  }
}
