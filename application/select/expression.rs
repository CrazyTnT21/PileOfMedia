use crate::select::comparison::Comparison;
use crate::select::condition::Condition;

#[derive(Debug)]
pub struct Expression<'a> {
  pub condition: Condition<'a>,
  pub ands: Vec<Expression<'a>>,
  pub ors: Vec<Expression<'a>>,
}

impl<'a> Expression<'a> {
  pub fn new(condition: Condition<'a>) -> Expression<'a> {
    Expression { condition, ands: vec![], ors: vec![] }
  }
  pub fn and(mut self, expression: Expression<'a>) -> Self {
    self.ands.push(expression);
    self
  }
  pub fn or(mut self, expression: Expression<'a>) -> Self {
    self.ors.push(expression);
    self
  }
  pub fn fmt(&self, count: &'a mut usize) -> String {
    fn add_one(value: &mut usize) -> &mut usize {
      *value += 1;
      value
    }
    let con = match &self.condition {
      Condition::Column(a, b) => format!("{}.{} = {}.{}", a.0, a.1, b.0, b.1),
      Condition::Value(a, b) => match b {
        Comparison::Equal(_) => format!("{}.{} = ${}", a.0, a.1, add_one(count)),
        Comparison::IsNull => format!("{}.{} IS NULL", a.0, a.1),
        Comparison::IsNotNull => format!("{}.{} IS NOT NULL", a.0, a.1),
        Comparison::NotEqual(_) => format!("{}.{} != ${}", a.0, a.1, add_one(count)),
        Comparison::ILike(_) => format!("{}.{} ILIKE ${}", a.0, a.1, add_one(count)),
        Comparison::In(value) => format!("{}.{} in ({})", a.0, a.1, value.iter().map(|_| format!("${}",add_one(count))).collect::<Vec<String>>().join(",")),
        Comparison::Bigger(_) => format!("{}.{} > ${}", a.0, a.1, add_one(count)),
        Comparison::BiggerEqual(_) => format!("{}.{} >= ${}", a.0, a.1, add_one(count)),
        Comparison::Less(_) => format!("{}.{} < ${}", a.0, a.1, add_one(count)),
        Comparison::LessEqual(_) => format!("{}.{} >= ${}", a.0, a.1, add_one(count)),
      }
    };
    let ands = self.ands.iter().map(|x| format!("AND ({})", x.fmt(count))).collect::<Vec<String>>().join(" ");
    let ors = self.ors.iter().map(|x| format!("OR ({})", x.fmt(count))).collect::<Vec<String>>().join(" ");
    format!("{} {} {}", con, ands, ors)
  }
}
