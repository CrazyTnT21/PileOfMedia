use crate::select::comparison::Comparison;

#[derive(Debug)]
pub enum Condition<'a> {
  Column((&'a str, &'a str), (&'a str, &'a str)),
  Value((&'a str, &'a str), Comparison<'a>),
}
