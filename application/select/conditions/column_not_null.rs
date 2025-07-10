use crate::select::condition::Condition;
use crate::select::selector::Selector;

pub struct ColumnNotNull<'a> {
  selector: Box<dyn Selector + 'a>,
}

impl<'a> ColumnNotNull<'a> {
  pub fn new(selector: impl Selector + 'a) -> ColumnNotNull<'a> {
    ColumnNotNull {
      selector: Box::new(selector),
    }
  }
}

impl Condition for ColumnNotNull<'_> {
  fn sql(&self, _value_index: &mut usize) -> String {
    let selector = self.selector.sql();
    format!("{selector} IS NOT NULL")
  }
}
