use crate::select::condition::Condition;
use crate::select::expression::IntoSql;
use crate::select::selector::Selector;

pub struct ValueILike<'a> {
  selector: Box<dyn Selector + 'a>,
  value: &'a String,
}

impl<'a> ValueILike<'a> {
  pub fn new(selector: impl Selector + 'a, value: &'a String) -> ValueILike<'a> {
    ValueILike {
      selector: Box::new(selector),
      value,
    }
  }
}

impl Condition for ValueILike<'_> {
  fn sql(&self, value_index: &mut usize) -> String {
    let selector = self.selector.sql();
    format!("{} ILIKE ${}", selector, crate::select::expression::next(value_index))
  }

  fn values(&self) -> Vec<&IntoSql> {
    vec![self.value]
  }
}
