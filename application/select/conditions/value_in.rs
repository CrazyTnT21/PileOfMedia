use crate::select::expression::{ConditionTrait, IntoSql};
use crate::select::selector::Selector;
use crate::select::to_sql_value::{ToSqlValue};

pub struct ValueIn<'a> {
  selector: Box<dyn Selector + 'a>,
  values: Box<dyn ToSqlValue<'a> + 'a>,
}

impl<'a> ValueIn<'a> {
  pub fn new(selector: impl Selector + 'a, values: impl ToSqlValue<'a> + 'a) -> ValueIn<'a> {
    ValueIn { selector: Box::new(selector), values: Box::new(values) }
  }
}

impl ConditionTrait for ValueIn<'_> {
  fn sql(&self, value_index: &mut usize) -> String {
    let selector = self.selector.sql();
    let values = self.values.sql(value_index);
    format!("{} IN ({})", selector, values)
  }

  fn values(&self) -> Vec<&IntoSql> {
    self.values.values()
  }
}
