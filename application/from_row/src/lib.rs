use tokio_postgres::Row;

pub use from_row_macros::query_row;
pub use from_row_macros::FromRow;

use crate::postgres_type::TypeKind;

pub mod postgres_type;

pub trait FromRow {
  type DbType;
  const COLUMN_COUNT: usize;
  fn from_row(row: &Row, from: usize) -> Self::DbType;
}

pub trait RowColumns<T: FromRow = Self>: FromRow {
  const COLUMNS: &'static [(&'static str, &'static [TypeKind])];
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
  const COLUMNS: &'static [(&'static str, &'static [TypeKind])] = T::COLUMNS;
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
  use tokio_postgres::types::Type;

  use crate::postgres_type::{PostgresType, TypeKind};
  use crate::FromRowOption;
  use crate::{from_row_impl, FromRow};

  from_row_impl!(NaiveDate);
  from_row_impl!(NaiveTime);
  from_row_impl!(NaiveDateTime);
  impl PostgresType for NaiveDate {
    const POSTGRES_TYPES: &'static [TypeKind] = &[TypeKind::Postgres(Type::DATE)];
  }
  impl PostgresType for NaiveTime {
    const POSTGRES_TYPES: &'static [TypeKind] = &[TypeKind::Postgres(Type::TIME)];
  }
  impl PostgresType for NaiveDateTime {
    const POSTGRES_TYPES: &'static [TypeKind] = &[TypeKind::Postgres(Type::TIMESTAMP)];
  }
}

#[cfg(feature = "testing")]
pub mod testing {
  use std::error::Error;
  use std::sync::Mutex;

  use bb8_postgres::bb8::Pool;
  use bb8_postgres::PostgresConnectionManager;
  use testcontainers::core::{IntoContainerPort, WaitFor};
  use testcontainers::runners::AsyncRunner;
  use testcontainers::{ContainerAsync, GenericImage, ImageExt};
  use tokio_postgres::{Column, NoTls, Statement};

  use crate::postgres_type::TypeKind;

  use super::{RowColumns, Table};

  static CONTAINER: tokio::sync::Mutex<Option<ContainerAsync<GenericImage>>> = tokio::sync::Mutex::const_new(None);
  static COUNT: Mutex<usize> = Mutex::new(0);

  pub async fn from_row_test<T: RowColumns + Table>() {
    let result = wrapper::<T>().await;
    {
      let count = {
        let mut count = COUNT.lock().unwrap();
        *count -= 1;
        *count
      };
      if count == 0 {
        let mut lock = CONTAINER.lock().await;
        *lock = None;
      }
    }
    match result {
      Ok(statement) => {
        for (i, column) in statement.columns().iter().enumerate() {
          for column_type in T::COLUMNS[i].1 {
            if validate_column(column, column_type) {
              break;
            }
          }
        }
      }
      Err(e) => {
        panic!("{}", e);
      }
    };
  }
  fn validate_column(column: &Column, column_type: &TypeKind) -> bool {
    match column_type {
      TypeKind::Postgres(post) => {
        if column.type_() == post {
          return true;
        }
        panic!(
          "column: {} ({}) does not match the struct type ({})",
          column.name(),
          column.type_(),
          post
        );
      }
      TypeKind::SimpleType { name, .. } => {
        if column.name() == *name {
          return true;
        }
        panic!(
          "column: {} ({}) does not match the struct type ({})",
          column.name(),
          column.type_(),
          name
        );
      }
    };
  }
  async fn wrapper<T: RowColumns + Table>() -> Result<Statement, Box<dyn Error>> {
    {
      let mut count = COUNT.lock()?;
      *count += 1;
    }
    {
      let mut lock = CONTAINER.lock().await;

      if !lock.is_some() {
        *lock = Some(create_image().await?);
      }
    }

    let manager = PostgresConnectionManager::new_from_stringlike(
      "postgresql://postgres:Placeholder@localhost:9876/collectiondb",
      NoTls,
    )?;
    let pool = Pool::builder().build(manager).await?;
    let connection = pool.get().await?;
    let result = connection
      .prepare(&format!(
        "SELECT {} FROM {}",
        T::COLUMNS.iter().map(|x| x.0).collect::<Vec<&'static str>>().join(","),
        T::TABLE_NAME
      ))
      .await;
    match result {
      Ok(rows) => Ok(rows),
      Err(err) => Err(err.as_db_error().ok_or("db error missing")?.message())?,
    }
  }

  async fn create_image() -> testcontainers::core::error::Result<ContainerAsync<GenericImage>> {
    GenericImage::new("pileofmedia-db", "latest")
      .with_wait_for(WaitFor::message_on_stdout(
        "database system is ready to accept connections",
      ))
      .with_mapped_port(9876, 5432.tcp())
      .with_network("pileofmedia")
      .with_env_var("POSTGRES_PASSWORD", "Placeholder")
      .start()
      .await
  }
}
