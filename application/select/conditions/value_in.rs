use crate::select::condition::Condition;
use crate::select::expression::IntoSql;
use crate::select::selector::Selector;
use crate::select::to_sql_value::ToSqlValue;

pub struct ValueIn<'a> {
  selector: Box<dyn Selector + 'a>,
  values: Box<dyn ToSqlValue<'a> + 'a>,
}

impl<'a> ValueIn<'a> {
  pub fn new(selector: impl Selector + 'a, values: impl ToSqlValue<'a> + 'a) -> ValueIn<'a> {
    ValueIn {
      selector: Box::new(selector),
      values: Box::new(values),
    }
  }
}

impl Condition for ValueIn<'_> {
  fn sql(&self, value_index: &mut usize) -> String {
    if self.values.values().is_empty() {
      return "FALSE".to_string();
    }
    let selector = self.selector.sql();
    let values = self.values.sql(value_index);
    format!("{selector} IN ({values})")
  }

  fn values(&self) -> Vec<&IntoSql<'_>> {
    self.values.values()
  }
}
