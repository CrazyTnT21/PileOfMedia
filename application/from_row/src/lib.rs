use tokio_postgres::Row;
use tokio_postgres::types::FromSql;

pub use from_row_macros::FromRow;
pub use from_row_macros::query_row;

pub trait FromRow {
  type DbType;
  const COLUMN_COUNT: usize;
  fn from_row(row: &Row, from: usize) -> Self::DbType;
  fn from_row_optional(row: &Row, from: usize) -> Option<Self::DbType>;
}

pub trait RowColumns<T: FromRow = Self>: FromRow {
  fn columns() -> Vec<&'static str>;
}

impl<T> FromRow for T where T: for<'a> FromSql<'a> {
  type DbType = T;
  const COLUMN_COUNT: usize = 1;

  fn from_row(row: &Row, from: usize) -> Self::DbType {
    row.get(from)
  }

  fn from_row_optional(row: &Row, from: usize) -> Option<Self::DbType> {
    row.try_get(from).ok()
  }
}
