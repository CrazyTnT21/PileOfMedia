use std::fmt::{Display, Formatter};
use crate::select::selector::Selector;

pub struct OrderBy<'a> {
  pub selector: Box<dyn Selector + 'a>,
  pub direction: Direction,
  pub nulls_order: Option<NullsOrder>,
}

impl Display for OrderBy<'_> {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{} {} {}", self.selector.sql(), self.direction, self.nulls_order.as_ref().map(|x| format!("NULLS {x}")).unwrap_or_default())
  }
}

pub enum NullsOrder {
  First,
  Last,
}

impl Display for NullsOrder {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", match self {
      NullsOrder::First => "FIRST",
      NullsOrder::Last => "LAST"
    })
  }
}

pub enum Direction {
  Ascending,
  Descending,
}

impl Display for Direction {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", match self {
      Direction::Ascending => "ASC",
      Direction::Descending => "DESC"
    })
  }
}
