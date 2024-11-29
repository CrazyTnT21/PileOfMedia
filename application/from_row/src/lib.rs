use tokio_postgres::Row;

pub use from_row_macros::query_row;
pub use from_row_macros::FromRow;

pub trait FromRow {
  type DbType;
  const COLUMN_COUNT: usize;
  fn from_row(row: &Row, from: usize) -> Self::DbType;
}

pub trait RowColumns<T: FromRow = Self>: FromRow {
  const COLUMNS: &'static [&'static str];
}

pub trait Table {
  const TABLE_NAME: &'static str;
}

pub trait FromRowOption<T: FromRow = Self> {
  fn from_row_optional(row: &Row, from: usize) -> Option<T::DbType>;
}

impl<T: FromRow<DbType = T> + FromRowOption> FromRow for Option<T> {
  type DbType = Option<T>;
  const COLUMN_COUNT: usize = T::COLUMN_COUNT;

  fn from_row(row: &Row, from: usize) -> Self::DbType {
    T::from_row_optional(row, from)
  }
}
macro_rules! from_row_tuple {
    ($first_generic: tt,$($generics: tt),*) => {
      impl<$first_generic: FromRow<DbType=$first_generic>,$($generics: FromRow<DbType=$generics>),*> FromRow for ($first_generic,$($generics),*) {
      type DbType = ($first_generic,$($generics),*);
      const COLUMN_COUNT: usize = $first_generic::COLUMN_COUNT$(+$generics::COLUMN_COUNT)*;

      fn from_row(row: &Row, from: usize) -> Self::DbType {
          let mut current_start = from;
        let mut start = |x| {
          let current = current_start;
          current_start += x;
          current
        };
        (
          $first_generic::from_row(row,start($first_generic::COLUMN_COUNT)),
          $($generics::from_row(row,start($generics::COLUMN_COUNT))),*
        )
      }
      }

    }
}
impl FromRow for () {
  type DbType = ();

  const COLUMN_COUNT: usize = 0;
  fn from_row(_: &Row, _: usize) -> Self::DbType {}
}

impl<T: FromRow<DbType = T> + RowColumns + FromRowOption> RowColumns for Option<T> {
  const COLUMNS: &'static [&'static str] = T::COLUMNS;
}
from_row_tuple!(T,);
from_row_tuple!(T, T1);
from_row_tuple!(T, T1, T2);
from_row_tuple!(T, T1, T2, T3);
from_row_tuple!(T, T1, T2, T3, T4);
from_row_tuple!(T, T1, T2, T3, T4, T5);
from_row_tuple!(T, T1, T2, T3, T4, T5, T6);
from_row_tuple!(T, T1, T2, T3, T4, T5, T6, T7);
from_row_tuple!(T, T1, T2, T3, T4, T5, T6, T7, T8);

#[macro_export]
macro_rules! from_row_impl {
  ($x: tt) => {
    impl FromRow for $x {
      type DbType = $x;
      const COLUMN_COUNT: usize = 1;

      fn from_row(row: &tokio_postgres::Row, from: usize) -> Self::DbType {
        row.get(from)
      }
    }
    impl FromRowOption for $x {
      fn from_row_optional(row: &tokio_postgres::Row, from: usize) -> Option<<$x as FromRow>::DbType> {
        row.try_get(from).ok()
      }
    }
  };
}

from_row_impl!(i8);
from_row_impl!(i16);
from_row_impl!(i32);
from_row_impl!(i64);
from_row_impl!(u32);
from_row_impl!(f32);
from_row_impl!(f64);
from_row_impl!(bool);
from_row_impl!(String);

#[cfg(feature = "chrono")]
mod chrono_from {
  use chrono::{NaiveDate, NaiveDateTime, NaiveTime};

  use crate::FromRowOption;
  use crate::{from_row_impl, FromRow};

  from_row_impl!(NaiveDate);
  from_row_impl!(NaiveTime);
  from_row_impl!(NaiveDateTime);
}
