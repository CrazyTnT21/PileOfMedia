use crate::select::comparison::Comparison;

#[derive(Debug, Clone)]
pub enum Condition<'a> {
  Column((&'a str, &'a str), (&'a str, &'a str)),
  Value((&'a str, &'a str), Comparison<'a>),
}
