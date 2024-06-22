use crate::select::expression::ConditionTrait;
use crate::select::selector::Selector;

pub struct ColumnEqual<'a> {
  selector: Box<dyn Selector + 'a>,
  second_selector: Box<dyn Selector + 'a>,
}

impl ConditionTrait for ColumnEqual<'_> {
  fn sql(&self, _value_index: &mut usize) -> String {
    let selector = self.selector.sql();
    let second_selector = self.second_selector.sql();
    format!("{selector} = {second_selector}")
  }
}

impl<'a> ColumnEqual<'a> {
  pub fn new(selector: impl Selector + 'a, second_selector: impl Selector + 'a) -> Self {
    Self {
      selector: Box::new(selector),
      second_selector: Box::new(second_selector),
    }
  }
}
