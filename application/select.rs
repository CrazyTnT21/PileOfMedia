use std::collections::HashMap;
use std::error::Error;
use std::marker::PhantomData;

use tokio_postgres::types::ToSql;
use tokio_postgres::Client;

use domain::pagination::Pagination;
use from_row::{FromRow, Table};

use crate::select::column_table::{ColumnTable, SelectElement};
use crate::select::combined_tuple::CombinedType;
use crate::select::expression::Expression;
use crate::select::join::{Join, JoinType};
use crate::select::order_by::{Direction, NullsOrder, OrderBy};
use crate::select::selector::Selector;

mod column_table;
pub mod combined_tuple;
pub mod condition;
pub mod conditions;
pub mod expression;
pub mod join;
pub mod order_by;
pub mod selector;
pub mod to_sql_value;

//TODO: Prepared version
pub struct Select<'a, T: FromRow<DbType = T> + CombinedType> {
  marker: PhantomData<T>,
  offset: Option<usize>,
  limit: Option<usize>,
  from: &'a str,
  alias: Option<&'a str>,
  columns: Vec<SelectElement<'a>>,
  joins: Vec<Join<'a>>,
  wheres: Vec<Expression<'a>>,
  distinct: HashMap<&'a str, Vec<&'a str>>,
  group_by: HashMap<&'a str, Vec<&'a str>>,
  order_by: Vec<OrderBy<'a>>,
  having: Vec<Expression<'a>>,
}

impl<'a> Select<'a, ()> {
  pub fn new<T: Table>() -> Select<'a, ()> {
    Self::new_raw(T::TABLE_NAME)
  }
  pub fn new_raw(from: &'a str) -> Select<'a, ()> {
    Select {
      marker: PhantomData,
      from,
      alias: None,
      columns: vec![],
      joins: vec![],
      offset: None,
      limit: None,
      wheres: vec![],
      distinct: HashMap::new(),
      group_by: HashMap::new(),
      order_by: vec![],
      having: vec![],
    }
  }
}

impl<'a, T: from_row::FromRow<DbType = T> + CombinedType> Select<'a, T> {
  pub const fn alias(mut self, alias: &'a str) -> Self {
    self.alias = Some(alias);
    self
  }
  pub fn column<C: FromRow<DbType = C>>(
    mut self,
    from: &'a str,
    column: &'a str,
  ) -> Select<'a, <T as CombinedType>::Combined<C>>
  where
    <T as CombinedType>::Combined<C>: FromRow<DbType = <T as CombinedType>::Combined<C>>,
  {
    self.columns.push(SelectElement::Column(ColumnTable {
      columns: vec![column],
      alias: from,
    }));
    self.create_new_select::<C>()
  }
  fn create_new_select<C>(self) -> Select<'a, <T as CombinedType>::Combined<C>>
  where
    <T as CombinedType>::Combined<C>: FromRow<DbType = <T as CombinedType>::Combined<C>>,
  {
    Select::<<T as CombinedType>::Combined<C>> {
      marker: PhantomData,
      offset: self.offset,
      limit: self.limit,
      from: self.from,
      alias: self.alias,
      columns: self.columns,
      joins: self.joins,
      wheres: self.wheres,
      distinct: self.distinct,
      group_by: self.group_by,
      order_by: self.order_by,
      having: self.having,
    }
  }
  pub fn columns<C: from_row::RowColumns + FromRow<DbType = C>>(
    mut self,
    from: &'a str,
  ) -> Select<'a, <T as CombinedType>::Combined<C>>
  where
    <T as CombinedType>::Combined<C>: FromRow<DbType = <T as CombinedType>::Combined<C>>,
  {
    self.columns.push(SelectElement::Column(ColumnTable {
      columns: C::COLUMNS.iter().map(|x| x.0).collect::<Vec<&'static str>>(),
      alias: from,
    }));
    self.create_new_select::<C>()
  }
  pub fn columns_table<C: from_row::RowColumns + FromRow<DbType = C> + Table>(
    mut self,
  ) -> Select<'a, <T as CombinedType>::Combined<C>>
  where
    <T as CombinedType>::Combined<C>: FromRow<DbType = <T as CombinedType>::Combined<C>>,
  {
    self.columns.push(SelectElement::Column(ColumnTable {
      columns: C::COLUMNS.iter().map(|x| x.0).collect::<Vec<&'static str>>(),
      alias: C::TABLE_NAME,
    }));
    self.create_new_select::<C>()
  }
  pub fn distinct_on(mut self, from: &'a str, column: &'a str) -> Self {
    match self.distinct.get_mut(from) {
      None => {
        self.distinct.insert(from, vec![column]);
      }
      Some(value) => {
        value.push(column);
      }
    };
    self
  }
  pub fn group_by(mut self, from: &'a str, column: &'a str) -> Self {
    match self.group_by.get_mut(from) {
      None => {
        self.group_by.insert(from, vec![column]);
      }
      Some(value) => {
        value.push(column);
      }
    };
    self
  }
  pub fn having(mut self, expression: Expression<'a>) -> Self {
    self.having.push(expression);
    self
  }
  pub fn where_expression(mut self, expression: Expression<'a>) -> Self {
    self.wheres.push(expression);
    self
  }

  pub const fn offset(mut self, offset: usize) -> Self {
    self.offset = Some(offset);
    self
  }

  pub const fn limit(mut self, limit: usize) -> Self {
    self.limit = Some(limit);
    self
  }

  /// Alias for limit() & offset()
  pub const fn pagination(self, pagination: Pagination) -> Self {
    self
      .limit(pagination.count as usize)
      .offset((pagination.count * pagination.page) as usize)
  }

  pub fn left_join<A: Table>(mut self, alias: Option<&'a str>, expression: Expression<'a>) -> Self {
    self
      .joins
      .push(Join::new(A::TABLE_NAME, JoinType::Left, alias, expression));
    self
  }
  pub fn left_join_raw(mut self, table: &'a str, alias: Option<&'a str>, expression: Expression<'a>) -> Self {
    self.joins.push(Join::new(table, JoinType::Left, alias, expression));
    self
  }

  pub fn transform<A: CombinedType + FromRow<DbType = A>>(
    self,
    function: impl FnOnce(Self) -> Select<'a, A>,
  ) -> Select<'a, A> {
    function(self)
  }

  pub fn inner_join<A: Table>(mut self, alias: Option<&'a str>, expression: Expression<'a>) -> Self {
    self
      .joins
      .push(Join::new(A::TABLE_NAME, JoinType::Inner, alias, expression));
    self
  }
  pub fn inner_join_raw(mut self, table: &'a str, alias: Option<&'a str>, expression: Expression<'a>) -> Self {
    self.joins.push(Join::new(table, JoinType::Inner, alias, expression));
    self
  }

  pub fn join(mut self, join: Join<'a>) -> Self {
    self.joins.push(join);
    self
  }

  pub fn count(mut self) -> Select<'a, <T as CombinedType>::Combined<i64>>
  where
    <T as CombinedType>::Combined<i64>: FromRow<DbType = <T as CombinedType>::Combined<i64>>,
  {
    self.columns.push(SelectElement::Raw("COUNT(*)"));
    self.create_new_select::<i64>()
  }

  pub fn order_by(
    mut self,
    selector: impl Selector + 'a,
    direction: Direction,
    nulls_order: Option<NullsOrder>,
  ) -> Self {
    self.order_by.push(OrderBy {
      selector: Box::new(selector),
      direction,
      nulls_order,
    });
    self
  }

  fn columns_sql(&self) -> String {
    self
      .columns
      .iter()
      .map(|x| match x {
        SelectElement::Column(column_table) => column_table
          .columns
          .iter()
          .map(|y| format!("{}.{}", column_table.alias, y))
          .collect::<Vec<String>>()
          .join(","),
        SelectElement::Raw(raw) => (*raw).to_string(),
      })
      .collect::<Vec<String>>()
      .join(",")
  }

  fn distinct_sql(&self) -> Option<String> {
    if self.distinct.is_empty() {
      return None;
    }
    Some(format!(
      "DISTINCT ON ({})",
      self
        .distinct
        .iter()
        .map(|(from, columns)| columns
          .iter()
          .map(|x| format!("{from}.{x}"))
          .collect::<Vec<String>>()
          .join(","))
        .collect::<Vec<String>>()
        .join(",")
    ))
  }
  fn order_by_sql(&self) -> Option<String> {
    if self.order_by.is_empty() {
      return None;
    }

    let order_bys = self
      .order_by
      .iter()
      .map(std::string::ToString::to_string)
      .collect::<Vec<String>>()
      .join(",");
    Some(format!("ORDER BY {}", order_bys))
  }

  fn limit_sql(&self) -> Option<String> {
    self.limit.map(|x| format!("LIMIT {x}"))
  }

  fn offset_sql(&self) -> Option<String> {
    self.offset.map(|x| format!("OFFSET {x}"))
  }

  fn where_sql(&self, count: &'a mut usize) -> Option<String> {
    if self.wheres.is_empty() {
      return None;
    }

    let wheres = self
      .wheres
      .iter()
      .map(|expression| expression.sql(count))
      .collect::<Vec<String>>()
      .join(" AND ");
    Some(format!("WHERE {}", wheres))
  }

  fn group_by_sql(&self) -> Option<String> {
    if self.group_by.is_empty() {
      return None;
    }
    Some(format!(
      "GROUP BY {}",
      self
        .group_by
        .iter()
        .map(|(from, columns)| columns
          .iter()
          .map(|x| format!("{from}.{x}"))
          .collect::<Vec<String>>()
          .join(","))
        .collect::<Vec<String>>()
        .join(",")
    ))
  }

  fn having_sql(&self, count: &'a mut usize) -> Option<String> {
    if self.having.is_empty() {
      return None;
    }

    let having = self
      .having
      .iter()
      .map(|expression| expression.sql(count))
      .collect::<String>();
    Some(format!("HAVING {}", having))
  }

  fn join_sql(&self, count: &'a mut usize) -> String {
    self.joins.iter().map(|join| join.fmt(count)).collect::<String>()
  }

  fn query_sql(&self) -> String {
    let columns = self.columns_sql();
    let mut count = 1;
    let joins = self.join_sql(&mut count);
    let where_sql = self.where_sql(&mut count).unwrap_or_default();
    let group_by_sql = self.group_by_sql().unwrap_or_default();
    let having_sql = self.having_sql(&mut count).unwrap_or_default();
    let limit_sql = self.limit_sql().unwrap_or_default();
    let offset_sql = self.offset_sql().unwrap_or_default();
    let alias_sql = self.alias.unwrap_or_default();
    let distinct_sql = self.distinct_sql().unwrap_or_default();
    let order_by_sql = self.order_by_sql().unwrap_or_default();
    let from = self.from;

    format!("SELECT {distinct_sql} {columns} FROM {from} {alias_sql} {joins} {where_sql}{group_by_sql} {having_sql} {order_by_sql} {limit_sql} {offset_sql}")
  }

  pub async fn query(self, connection: &'a Client) -> Result<Vec<T>, Box<dyn Error>> {
    Ok(
      connection
        .query(&self.query_sql(), &self.values())
        .await?
        .into_iter()
        .map(|x| T::from_row(&x, 0))
        .collect::<Vec<T>>(),
    )
  }

  fn values(&'a self) -> Vec<&'a (dyn ToSql + Sync)> {
    let mut total: Vec<&'a (dyn ToSql + Sync)> = vec![];
    self.joins.iter().for_each(|x| total.append(&mut x.expression.values()));
    self.wheres.iter().for_each(|x| total.append(&mut x.values()));
    total
  }

  pub async fn get_single(self, connection: &'a Client) -> Result<Option<T>, Box<dyn Error>> {
    Ok(
      connection
        .query_opt(&self.query_sql(), &self.values())
        .await?
        .map(|x| T::from_row(&x, 0)),
    )
  }
}

impl<'a> Select<'a, ()> {
  /// # Panics
  ///
  /// Will panic if the database returns more than one row.
  pub async fn query_count(self, connection: &'a Client) -> Result<i64, Box<dyn Error>> {
    let result = self
      .count()
      .get_single(connection)
      .await?
      .expect("Count should return one row");
    Ok(result.0)
  }
}
impl<'a, T: from_row::FromRow<DbType = T>> Select<'a, (T,)> {
  pub async fn query_destruct(self, connection: &'a Client) -> Result<Vec<T>, Box<dyn Error>> {
    let result: Vec<T> = self.query(connection).await?.into_iter().map(|(x,)| x).collect();
    Ok(result)
  }
}
