use crate::select::expression::Expression;

pub struct Join<'a> {
  pub table: &'a str,
  pub alias: Option<&'a str>,
  pub join_type: JoinType,
  pub expression: Expression<'a>,
}

impl<'a> Join<'a> {
  pub fn new(table: &'a str, join_type: JoinType, alias: Option<&'a str>, expression: Expression<'a>) -> Join<'a> {
    Join { alias, table, join_type, expression }
  }
  pub fn fmt(&self, count: &mut usize) -> String {
    let join_type = match self.join_type {
      JoinType::Inner => "INNER",
      JoinType::Left => "LEFT"
    };
    format!("{} JOIN {} {} ON {}", join_type, self.table, self.alias.unwrap_or(""), &self.expression.sql(count))
  }
}

#[derive(Debug, Clone)]
pub enum JoinType {
  Inner,
  Left,
}
