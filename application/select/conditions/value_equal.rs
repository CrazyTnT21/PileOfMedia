use crate::select::condition::Condition;
use crate::select::expression::IntoSql;
use crate::select::selector::Selector;
use crate::select::to_sql_value::ToSqlValue;

pub struct ValueEqual<'a> {
  selector: Box<dyn Selector + 'a>,
  value: Box<dyn ToSqlValue<'a> + 'a>,
}

impl Condition for ValueEqual<'_> {
  fn sql(&self, value_index: &mut usize) -> String {
    let selector = self.selector.sql();
    format!("{} = {}", selector, self.value.sql(value_index))
  }
  fn values(&self) -> Vec<&IntoSql<'_>> {
    self.value.values()
  }
}

impl<'a> ValueEqual<'a> {
  pub fn new(selector: impl Selector + 'a, value: impl ToSqlValue<'a> + 'a) -> Self {
    Self {
      selector: Box::new(selector),
      value: Box::new(value),
    }
  }
}
