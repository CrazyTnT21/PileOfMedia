use crate::select::condition::Condition;
use crate::select::selector::Selector;

pub struct ColumnNull<'a> {
  selector: Box<dyn Selector + 'a>,
}

impl<'a> ColumnNull<'a> {
  pub fn new(selector: impl Selector + 'a) -> ColumnNull<'a> {
    ColumnNull {
      selector: Box::new(selector),
    }
  }
}

impl Condition for ColumnNull<'_> {
  fn sql(&self, _value_index: &mut usize) -> String {
    let selector = self.selector.sql();
    format!("{} IS NULL", selector)
  }
}
