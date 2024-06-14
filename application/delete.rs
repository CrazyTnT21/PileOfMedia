use std::error::Error;
use std::fmt::{Display, Formatter};
use tokio_postgres::{Client, Transaction};
use tokio_postgres::types::ToSql;
use from_row::Table;
use crate::delete::DeleteError::PostgresError;
use crate::select::expression::Expression;

pub struct Delete<'a> {
  from: &'a str,
  where_condition: Expression<'a>,
}

impl<'a> Delete<'a> {
  pub fn new<T: Table>(where_condition: Expression<'a>) -> Delete<'a> {
    Self::new_raw(T::TABLE_NAME, where_condition)
  }
  pub fn new_raw(from: &'a str, where_condition: Expression<'a>) -> Delete<'a> {
    Delete {
      from,
      where_condition,
    }
  }
}

impl<'a> Delete<'a> {
  pub async fn execute_transaction(&self, transaction: &'a Transaction<'a>) -> Result<u64, DeleteError> {
    transaction.execute(&self.sql(), &self.values()).await.map_err(PostgresError)
  }
  pub async fn execute(&self, client: &'a Client) -> Result<u64, DeleteError> {
    client.execute(&self.sql(), &self.values()).await.map_err(PostgresError)
  }
  fn sql(&self) -> String {
    format!("DELETE FROM {} WHERE {}", self.from, self.where_condition.fmt(&mut 0))
  }
  fn values(&self) -> Vec<&'a (dyn ToSql + Sync)> {
    let mut result = vec![];
    Expression::values(&self.where_condition, &mut result);
    result
  }
}


#[derive(Debug)]
pub enum DeleteError {
  PostgresError(tokio_postgres::Error),
}


impl Display for DeleteError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      PostgresError(value) => std::fmt::Display::fmt(&value, f)
    }
  }
}

impl Error for DeleteError {}
