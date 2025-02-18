use crate::enums::db_language::DbLanguage;
use crate::enums::db_user_status::DbUserStatus;
use crate::select::expression::{next, IntoSql};
use chrono::{NaiveDate, NaiveTime};
use tokio_postgres::types::ToSql;

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

impl<'a, T: ToSqlValue<'a> + ToSql + 'a> ToSqlValue<'a> for Option<T> {
  fn values(&self) -> Vec<&IntoSql<'a>> {
    match self {
      None => vec![self],
      Some(value) => value.values(),
    }
  }
  fn sql(&self, index: &mut usize) -> String {
    match self {
      None => format!("${}", next(index)),
      Some(value) => value.sql(index),
    }
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
  };
}
to_value!(bool);
to_value!(i8);
to_value!(i16);
to_value!(i32);
to_value!(i64);
to_value!(DbLanguage);
to_value!(DbUserStatus);
to_value!(NaiveDate);
to_value!(NaiveTime);
to_value!(String);
to_value!(&'a str);
to_value!(f32);
