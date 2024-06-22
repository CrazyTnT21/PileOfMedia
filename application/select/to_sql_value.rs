use tokio_postgres::types::ToSql;
use crate::enums::db_language::DbLanguage;
use crate::select::expression::{IntoSql, next};

pub trait ToSqlValue<'a>: Send + Sync {
  fn values(&self) -> Vec<&IntoSql<'a>>;
  fn sql(&self, index: &mut usize) -> String {
    format!("${}", next(index))
  }
}

impl<'a, T: ToSqlValue<'a>, A: ToSqlValue<'a>> ToSqlValue<'a> for (T, A) {
  fn values(&self) -> Vec<&IntoSql<'a>> {
    let mut result = vec![];
    result.append(&mut self.0.values());
    result.append(&mut self.1.values());
    result
  }
  fn sql(&self, index: &mut usize) -> String {
    format!("({},{})", self.0.sql(index), self.1.sql(index))
  }
}

impl<'a, T: ToSqlValue<'a>> ToSqlValue<'a> for &'a Vec<T> {
  fn values(&self) -> Vec<&IntoSql<'a>> {
    let mut result = vec![];
    self.iter().for_each(|x| result.append(&mut x.values()));
    result
  }
  fn sql(&self, index: &mut usize) -> String {
    self.iter().map(|x| x.sql(index)).collect::<Vec<String>>().join(",")
  }
}

impl<'a, T: ToSqlValue<'a>> ToSqlValue<'a> for &'a [T] {
  fn values(&self) -> Vec<&IntoSql<'a>> {
    let mut result = vec![];
    self.iter().for_each(|x| result.append(&mut x.values()));
    result
  }
  fn sql(&self, index: &mut usize) -> String {
    self.iter().map(|x| x.sql(index)).collect::<Vec<String>>().join(",")
  }
}
macro_rules! to_value {
  ($t: ty) => {
    impl<'a> ToSqlValue<'a> for &$t {
  fn values(&self) -> Vec<&IntoSql<'a>> {
    vec![*self]
  }
}
impl<'a> ToSqlValue<'a> for $t {
  fn values(&self) -> Vec<&IntoSql<'a>> {
    vec![self]
  }
    }
  }
}
to_value!(i8);
to_value!(i16);
to_value!(i32);
to_value!(i64);
to_value!(DbLanguage);
to_value!(String);
to_value!(&'a str);

