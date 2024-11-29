use crate::select::expression::IntoSql;

pub trait Condition: Send + Sync {
  fn sql(&self, value_index: &mut usize) -> String;
  fn values(&self) -> Vec<&IntoSql> {
    Vec::new()
  }
}
