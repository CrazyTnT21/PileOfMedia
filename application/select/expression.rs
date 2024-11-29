use crate::select::condition::Condition;
use crate::select::conditions::value_equal::ValueEqual;
use crate::select::to_sql_value::ToSqlValue;
use tokio_postgres::types::ToSql;

pub fn next(value: &mut usize) -> usize {
  let last_value = *value;
  *value += 1;
  last_value
}

pub struct Expression<'a> {
  pub condition: Box<dyn Condition + 'a>,
  pub ands: Vec<Expression<'a>>,
  pub ors: Vec<Expression<'a>>,
}

impl<'a> Expression<'a> {
  pub fn new(condition: impl Condition + 'a) -> Expression<'a> {
    Self {
      condition: Box::new(condition),
      ands: vec![],
      ors: vec![],
    }
  }
  pub fn column_equal(from: &'a str, column: &'a str, value: impl ToSqlValue<'a> + 'a) -> Expression<'a> {
    Expression::new(ValueEqual::new((from, column), value))
  }

  pub fn values(&self) -> Vec<&IntoSql> {
    let mut result = self.condition.values();
    self.ands.iter().for_each(|x| result.append(&mut x.values()));
    self.ors.iter().for_each(|x| result.append(&mut x.values()));
    result
  }

  pub fn sql(&self, value_index: &mut usize) -> String {
    let condition = self.condition.sql(value_index);
    let ands = self
      .ands
      .iter()
      .map(|x| format!("AND ({})", x.sql(value_index)))
      .collect::<Vec<String>>()
      .join(" ");
    let ors = self
      .ors
      .iter()
      .map(|x| format!("OR ({})", x.sql(value_index)))
      .collect::<Vec<String>>()
      .join(" ");
    format!("{} {} {}", condition, ands, ors)
  }
  pub fn and(mut self, expression: Expression<'a>) -> Expression<'a> {
    self.ands.push(expression);
    self
  }
  pub fn or(mut self, expression: Expression<'a>) -> Expression<'a> {
    self.ors.push(expression);
    self
  }
}

pub type IntoSql<'a> = dyn ToSql + Sync + 'a;
