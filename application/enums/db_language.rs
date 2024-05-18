use from_row::FromRowOption;
use from_row::FromRow;
use std::error::Error;
use std::str::FromStr;

use serde::{Deserialize, Serialize};
use tokio_postgres::types::{FromSql, IsNull, Kind, to_sql_checked, ToSql, Type};
use tokio_postgres::types::private::BytesMut;

use domain::enums::language::Language;
use from_row::from_row_impl;

use crate::{convert, enum_from_sql};

#[derive(Serialize, Deserialize, Copy, PartialEq, Eq, Clone, Debug)]
pub enum DbLanguage {
  EN,
  DE,
  JA,
  ES,
  DA,
  NL,
  KO,
}
from_row_impl!(DbLanguage);
convert!(DbLanguage,Language,EN,DE,JA,ES,DA,NL,KO);
enum_from_sql!(DbLanguage,"language");

impl DbLanguage {
  pub fn code(&self) -> String {
    Into::<Language>::into(*self).language_code().to_uppercase()
  }
}
impl FromStr for DbLanguage{
  type Err = <Language as FromStr>::Err;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    Language::from_str(s).and_then(|x| Ok(DbLanguage::from(x)))
  }
}
impl ToSql for DbLanguage {
  fn to_sql(&self, _ty: &Type, out: &mut BytesMut) -> Result<IsNull, Box<dyn Error + Sync + Send>> where Self: Sized {
    out.extend_from_slice(&self.code().bytes().collect::<Vec<u8>>());
    Ok(IsNull::No)
  }

  fn accepts(ty: &Type) -> bool where Self: Sized {
    <DbLanguage as FromSql>::accepts(ty)
  }
  to_sql_checked!();
}
