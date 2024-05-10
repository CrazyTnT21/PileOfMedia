use std::error::Error;
use std::fmt::Display;
use std::marker::PhantomData;
use std::string::ToString;

use bb8_postgres::bb8::PooledConnection;
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::NoTls;
use tokio_postgres::types::ToSql;

use domain::pagination::Pagination;
use from_row::FromRow;

use crate::select::combined_tuple::CombinedType;
use crate::select::comparison::Comparison;
use crate::select::condition::Condition;
use crate::select::expression::Expression;
use crate::select::join::{Join, JoinType};

//TODO: Prepared version
//TODO: Group By
//TODO: In
#[derive(Debug)]
pub struct Select<'a, T: FromRow<DbType=T> + CombinedType>
{
  marker: PhantomData<T>,
  offset: Option<usize>,
  limit: Option<usize>,
  from: &'a str,
  alias: Option<&'a str>,
  columns: Vec<ColumnTable<'a>>,
  joins: Vec<Join<'a>>,
  wheres: Vec<Expression<'a>>,
}

impl<'a> Select<'a, ()> {
  pub fn new(from: &'a str) -> Select<'a, ()> {
    Select { marker: PhantomData, from, alias: None, columns: vec![], joins: vec![], offset: None, limit: None, wheres: vec![] }
  }
}

impl<'a, T: from_row::FromRow<DbType=T> + CombinedType> Select<'a, T> {
  pub fn alias(mut self, alias: &'a str) -> Self {
    self.alias = Some(alias);
    self
  }
  pub fn column<C: FromRow<DbType=C>>(mut self, from: &'a str, column: &'a str) -> Select<'a, <T as CombinedType>::Combined<C>> where <T as CombinedType>::Combined<C>: FromRow<DbType=<T as CombinedType>::Combined<C>> {
    self.columns.push(ColumnTable {
      columns: vec![column],
      alias: from,
    });
    Select::<<T as CombinedType>::Combined<C>> {
      marker: PhantomData,
      offset: self.offset,
      limit: self.limit,
      from: self.from,
      alias: self.alias,
      columns: self.columns,
      joins: self.joins,
      wheres: self.wheres,
    }
  }
  pub fn columns<C: from_row::RowColumns + FromRow<DbType=C>>(mut self, from: &'a str) -> Select<'a, <T as CombinedType>::Combined<C>> where <T as CombinedType>::Combined<C>: FromRow<DbType=<T as CombinedType>::Combined<C>> {
    self.columns.push(ColumnTable {
      columns: C::columns(),
      alias: from,
    });

    Select::<<T as CombinedType>::Combined<C>> {
      marker: PhantomData,
      offset: self.offset,
      limit: self.limit,
      from: self.from,
      alias: self.alias,
      columns: self.columns,
      joins: self.joins,
      wheres: self.wheres,
    }
  }
  pub fn where_expression(mut self, expression: Expression<'a>) -> Self {
    self.wheres.push(expression);
    self
  }

  pub fn offset(mut self, offset: usize) -> Self {
    self.offset = Some(offset);
    self
  }

  pub fn limit(mut self, limit: usize) -> Self {
    self.limit = Some(limit);
    self
  }

  /// Alias for limit() & offset()
  pub fn pagination(self, pagination: Pagination) -> Self {
    self
      .limit(pagination.count as usize)
      .offset((pagination.count * pagination.page) as usize)
  }

  pub fn left_join(mut self, table: &'a str, alias: Option<&'a str>, expression: Expression<'a>) -> Self {
    self.joins.push(Join::new(table, JoinType::Left, alias, expression));
    self
  }

  pub fn inner_join(mut self, table: &'a str, alias: Option<&'a str>, expression: Expression<'a>) -> Self {
    self.joins.push(Join::new(table, JoinType::Inner, alias, expression));
    self
  }

  pub fn join(mut self, join: Join<'a>) -> Self {
    self.joins.push(join);
    self
  }

  pub async fn count(&self, connection: &'a PooledConnection<'static, PostgresConnectionManager<NoTls>>) -> Result<i64, Box<dyn Error>> {
    let mut count = 0;
    let joins = self.join_sql(&mut count);
    let where_sql = self.where_sql(&mut count).unwrap_or_else(|| "".to_string());
    let limit_sql = self.limit_sql().unwrap_or_else(|| "".to_string());
    let offset_sql = self.offset_sql().unwrap_or_else(|| "".to_string());
    let alias_sql = self.alias.unwrap_or("");
    let query = format!("SELECT COUNT(*) FROM {} {alias_sql} {joins} {where_sql} {limit_sql} {offset_sql}", self.from);

    let rows = connection
      .query(&query, &self.values())
      .await?;

    if rows.len() == 0 {
      return Ok(0);
    }
    Ok(rows[0].get(0))
  }

  fn columns_sql(&self) -> String {
    self.columns
      .iter()
      .map(|x| x.columns
        .iter()
        .map(|y| format!("{}.{}", x.alias, y))
        .collect::<Vec<String>>().join(","))
      .collect::<Vec<String>>().join(",")
  }

  fn limit_sql(&self) -> Option<String> {
    self.limit.and_then(|x| Some(format!("LIMIT {x}")))
  }

  fn offset_sql(&self) -> Option<String> {
    self.offset.and_then(|x| Some(format!("OFFSET {x}")))
  }

  fn where_sql(&self, count: &'a mut usize) -> Option<String> {
    if self.wheres.len() == 0 {
      return None;
    }

    let wheres = self.wheres.iter()
      .map(|expression| expression.fmt(count))
      .collect::<Vec<String>>()
      .join("");
    Some(format!("WHERE {}", wheres))
  }

  fn join_sql(&self, count: &'a mut usize) -> String {
    self.joins
      .iter()
      .map(|join| join.fmt(count))
      .collect::<Vec<String>>().join("")
  }

  fn query_sql(&self) -> String {
    let columns = self.columns_sql();
    let mut count = 0;
    let joins = self.join_sql(&mut count);
    let where_sql = self.where_sql(&mut count).unwrap_or_else(|| "".to_string());
    let limit_sql = self.limit_sql().unwrap_or_else(|| "".to_string());
    let offset_sql = self.offset_sql().unwrap_or_else(|| "".to_string());
    let alias_sql = self.alias.unwrap_or("");

    format!("SELECT {columns} FROM {} {alias_sql} {joins} {where_sql} {limit_sql} {offset_sql}", self.from)
  }

  pub async fn query(self, connection: &'a PooledConnection<'static, PostgresConnectionManager<NoTls>>) -> Result<Vec<T>, Box<dyn Error>> {
    Ok(connection
      .query(&self.query_sql(), &self.values())
      .await?
      .into_iter()
      .map(|x| T::from_row(&x, 0))
      .collect::<Vec<T>>())
  }

  fn values(&self) -> Vec<&'a (dyn ToSql + Sync)> {
    let mut total: Vec<&'a (dyn ToSql + Sync)> = vec![];
    self.joins.iter().for_each(|x| Self::get_values_recursive(&mut total, &x.expression));
    self.wheres.iter().for_each(|x| Self::get_values_recursive(&mut total, &x));
    total
  }

  pub async fn get_single(self, connection: &'a PooledConnection<'static, PostgresConnectionManager<NoTls>>) -> Result<Option<T>, Box<dyn Error>> {
    Ok(connection
      .query_opt(&self.query_sql(), &self.values())
      .await?
      .and_then(|x| Some(T::from_row(&x, 0))))
  }

  fn get_values_recursive(current: &mut Vec<&'a (dyn ToSql + Sync)>, expression: &Expression<'a>) {
    match &expression.condition {
      Condition::Column(_, _) => {}
      Condition::Value(_, b) => match b {
        Comparison::Equal(value) => current.push(*value),
        Comparison::NotEqual(value) => current.push(*value),
        Comparison::IsNull => {}
        Comparison::IsNotNull => {}
        Comparison::ILike(value) => current.push(*value),
        Comparison::Bigger(value) => current.push(*value),
        Comparison::BiggerEqual(value) => current.push(*value),
        Comparison::Less(value) => current.push(*value),
        Comparison::LessEqual(value) => current.push(*value),
      }
    };
    expression.ands.iter().for_each(|mut x| Self::get_values_recursive(current, &mut x));
    expression.ors.iter().for_each(|mut x| Self::get_values_recursive(current, &mut x));
  }
}

#[derive(Debug)]
struct ColumnTable<'a> {
  columns: Vec<&'a str>,
  alias: &'a str,
}

// enum In<'a, T: from_row::FromRow<DbType=T> + CombinedType> {
//   Select(Select<'a, T>),
//   Array(&'a [&'a (dyn ToSql + Sync)]),
// }

