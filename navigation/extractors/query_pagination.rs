use serde::Deserialize;
use domain::pagination::Pagination;

#[derive(Deserialize, Debug)]
pub struct QueryPagination {
  #[serde(default)]
  pub page: u32,
  #[serde(default = "default_count")]
  pub count: u32,
}

fn default_count() -> u32 {
  50
}

impl From<Pagination> for QueryPagination {
  fn from(value: Pagination) -> Self {
    QueryPagination { count: value.count, page: value.page }
  }
}

impl From<QueryPagination> for Pagination {
  fn from(val: QueryPagination) -> Self {
    Pagination { count: val.count, page: val.page }
  }
}
