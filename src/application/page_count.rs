pub struct PageCount {
  pub page: u32,
  pub count: u32,
}

impl Default for PageCount {
  fn default() -> Self {
    PageCount { page: 0, count: 50 }
  }
}
