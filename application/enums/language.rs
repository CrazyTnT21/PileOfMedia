use from_row::FromRowOption;
use from_row::FromRow;
use std::error::Error;
use std::str::FromStr;

use serde::{Deserialize, Serialize};
use tokio_postgres::types::{FromSql, Kind, Type};

use domain::enums::language::Language;
use from_row::from_row_impl;

use crate::convert;

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

impl DbLanguage {
  pub fn code(&self) -> String {
    Into::<Language>::into(*self).language_code().to_uppercase()
  }
}

impl<'a> FromSql<'a> for DbLanguage {
  fn from_sql(_ty: &Type, raw: &'a [u8]) -> Result<Self, Box<dyn Error + Sync + Send>> {
    let language = Language::from_str(std::str::from_utf8(raw)?)?;
    Ok(language.into())
  }

  fn accepts(ty: &Type) -> bool {
    if ty.name() != "language" {
      return false;
    }
    let Kind::Enum(value) = ty.kind() else {
      return false;
    };
    for x in value {
      match Language::from_str(x) {
        Ok(_) => {}
        Err(_) => {
          return false;
        }
      }
    }

    true
  }
}

