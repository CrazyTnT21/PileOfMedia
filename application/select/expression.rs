use tokio_postgres::types::ToSql;

use crate::select::comparison::Comparison;
use crate::select::comparison::Comparison::{Bigger, BiggerEqual, Equal, ILike, In, IsNull, Less, LessEqual, NotEqual};
use crate::select::condition::Condition;
use crate::select::condition::Condition::{Column, Value};

#[derive(Debug)]
pub struct Expression<'a> {
  pub condition: Condition<'a>,
  pub ands: Vec<Expression<'a>>,
  pub ors: Vec<Expression<'a>>,
}

impl<'a> Expression<'a> {
  pub fn new(condition: Condition<'a>) -> Expression<'a> {
    Expression { condition, ands: vec![], ors: vec![] }
  }
  pub fn and(mut self, expression: Expression<'a>) -> Self {
    self.ands.push(expression);
    self
  }
  pub fn or(mut self, expression: Expression<'a>) -> Self {
    self.ors.push(expression);
    self
  }
  pub fn fmt(&self, count: &'a mut usize) -> String {
    fn add_one(value: &mut usize) -> &mut usize {
      *value += 1;
      value
    }
    let con = match &self.condition {
      Column(a, b) => format!("{}.{} = {}.{}", a.0, a.1, b.0, b.1),
      Value(a, b) => match b {
        Equal(_) => format!("{}.{} = ${}", a.0, a.1, add_one(count)),
        IsNull => format!("{}.{} IS NULL", a.0, a.1),
        Comparison::IsNotNull => format!("{}.{} IS NOT NULL", a.0, a.1),
        NotEqual(_) => format!("{}.{} != ${}", a.0, a.1, add_one(count)),
        ILike(_) => format!("{}.{} ILIKE ${}", a.0, a.1, add_one(count)),
        In(value) => format!("{}.{} in ({})", a.0, a.1, value.iter().map(|_| format!("${}", add_one(count))).collect::<Vec<String>>().join(",")),
        Bigger(_) => format!("{}.{} > ${}", a.0, a.1, add_one(count)),
        BiggerEqual(_) => format!("{}.{} >= ${}", a.0, a.1, add_one(count)),
        Less(_) => format!("{}.{} < ${}", a.0, a.1, add_one(count)),
        LessEqual(_) => format!("{}.{} >= ${}", a.0, a.1, add_one(count)),
      }
    };
    let ands = self.ands.iter().map(|x| format!("AND ({})", x.fmt(count))).collect::<Vec<String>>().join(" ");
    let ors = self.ors.iter().map(|x| format!("OR ({})", x.fmt(count))).collect::<Vec<String>>().join(" ");
    format!("{} {} {}", con, ands, ors)
  }
  pub fn column_null(table: &'a str, column: &'a str) -> Expression<'a> {
    Expression::new(Value((table, column), IsNull))
  }
  pub fn column_equal(table: &'a str, column: &'a str, value: &'a (dyn ToSql + Sync)) -> Expression<'a> {
    Expression::new(Value((table, column), Equal(value)))
  }
  pub fn values(expression: &Expression<'a>, current: &mut Vec<&'a (dyn ToSql + Sync)>) {
    match &expression.condition {
      Column(_, _) => {}
      Value(_, b) => match b {
        Equal(value) => current.push(*value),
        NotEqual(value) => current.push(*value),
        IsNull => {}
        Comparison::IsNotNull => {}
        ILike(value) => current.push(*value),
        In(value) => value.iter().for_each(|x| current.push(*x)),
        Bigger(value) => current.push(*value),
        BiggerEqual(value) => current.push(*value),
        Less(value) => current.push(*value),
        LessEqual(value) => current.push(*value),
      }
    };
    expression.ands.iter().for_each(|x| Self::values(x, current));
    expression.ors.iter().for_each(|x| Self::values(x, current));
  }
}
