use tokio_postgres::Row;

pub use from_row_macros::FromRow;
pub use from_row_macros::query_row;

pub trait FromRow {
  type DbType;
  fn from_row(row: &Row, from: usize) -> Self::DbType;
  fn from_row_optional(row: &Row, from: usize) -> Option<Self::DbType>;
}

pub trait RowColumns<T: FromRow = Self>: FromRow {
  fn columns() -> Vec<&'static str>;
}

