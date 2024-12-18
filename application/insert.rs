use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

use tokio_postgres::types::ToSql;
use tokio_postgres::{Client, Transaction};

use crate::select::to_sql_value::ToSqlValue;
use from_row::Table;

pub struct Insert<'a, const U: usize> {
  into: &'a str,
  columns: [&'a str; U],
  values: Vec<[&'a dyn ToSqlValue<'a>; U]>,
}

impl<'a, const U: usize> Insert<'a, U> {
  pub const fn new<T: Table>(columns: [&'a str; U]) -> Insert<'a, U> {
    Self::new_raw(T::TABLE_NAME, columns)
  }
  pub const fn new_raw(into: &'a str, columns: [&'a str; U]) -> Insert<'a, U> {
    Insert {
      into,
      columns,
      values: Vec::new(),
    }
  }
  pub fn values(mut self, values: [&'a dyn ToSqlValue<'a>; U]) -> Self {
    self.values.push(values);
    self
  }
  pub fn values_ref(&mut self, values: [&'a dyn ToSqlValue<'a>; U]) -> &Self {
    self.values.push(values);
    self
  }

  pub async fn execute(&self, connection: &'a Client) -> Result<u64, InsertError> {
    connection
      .execute(&self.sql(), &self.sql_values())
      .await
      .map_err(InsertError::PostgresError)
  }

  pub async fn execute_transaction(&self, transaction: &'a Transaction<'a>) -> Result<u64, InsertError> {
    transaction
      .execute(&self.sql(), &self.sql_values())
      .await
      .map_err(InsertError::PostgresError)
  }

  pub async fn returning<T: for<'b> tokio_postgres::types::FromSql<'b>>(
    &self,
    column: &'a str,
    connection: &'a Client,
  ) -> Result<T, InsertError> {
    if self.values.len() > 1 {
      return Err(InsertError::ReturningMoreThanOne);
    }

    let result = connection
      .query_one(&self.returning_sql(column), &self.sql_values())
      .await
      .map_err(InsertError::PostgresError)?;
    Ok(result.get::<'_, _, T>(0))
  }

  pub async fn returning_transaction<T: for<'b> tokio_postgres::types::FromSql<'b>>(
    &self,
    column: &'a str,
    transaction: &'a Transaction<'a>,
  ) -> Result<T, InsertError> {
    if self.values.len() > 1 {
      return Err(InsertError::ReturningMoreThanOne);
    }
    let result = transaction
      .query_one(&self.returning_sql(column), &self.sql_values())
      .await
      .map_err(InsertError::PostgresError)?;
    Ok(result.get::<'_, _, T>(0))
  }

  pub fn sql(&self) -> String {
    let into = self.into;
    if U == 0 {
      return format!("INSERT INTO {into} DEFAULT VALUES;");
    }

    let columns = self.columns_sql();
    let values = self.values_sql();
    format!(r"INSERT INTO {into}({columns}) values {values}")
  }

  pub fn returning_sql(&self, column: &'a str) -> String {
    let into = self.into;
    if U == 0 {
      return format!("INSERT INTO {into} DEFAULT VALUES RETURNING {column};");
    }

    let columns = self.columns_sql();
    let values = self.values_sql();
    format!(r"INSERT INTO {into}({columns}) values {values} returning {column};")
  }

  fn columns_sql(&self) -> String {
    self.columns.join(",")
  }

  fn values_sql(&self) -> String {
    let mut total = 0;
    self
      .values
      .iter()
      .map(|x| {
        let result = (1..x.len() + 1)
          .collect::<Vec<usize>>()
          .iter()
          .map(|_| {
            total += 1;
            format!("${}", total)
          })
          .collect::<Vec<String>>()
          .join(",");
        format!("({result})")
      })
      .collect::<Vec<String>>()
      .join(",")
  }

  fn sql_values(&self) -> Vec<&'a (dyn ToSql + Sync)> {
    let mut result = vec![];
    self.values.iter().for_each(|x| {
      x.iter().for_each(|x| {
        result.append(&mut x.values());
      });
    });
    result
  }
}

#[derive(Debug)]
pub enum InsertError {
  ReturningMoreThanOne,
  PostgresError(tokio_postgres::Error),
}

impl Display for InsertError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      InsertError::ReturningMoreThanOne => write!(f, "Returning is not supported for more than 1 value at a time"),
      InsertError::PostgresError(value) => std::fmt::Display::fmt(&value, f),
    }
  }
}

impl Error for InsertError {}
