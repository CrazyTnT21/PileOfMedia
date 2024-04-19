use std::error::Error;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

use serde::{Deserialize, Serialize};
use crate::enums::language::Language::{DE, EN, ES, JA};


#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum Language {
  EN,
  DE,
  JA,
  ES,
}

impl Display for Language {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", match self {
      EN => "English",
      DE => "Deutsch",
      JA => "日本語",
      ES => "Español",
    })
  }
}

impl Language {
  pub fn language_code(&self) -> &str {
    match self {
      EN => "en",
      DE => "de",
      JA => "ja",
      ES => "es",
    }
  }
}

impl FromStr for Language {
  type Err = Box<dyn Error>;

  fn from_str(value: &str) -> Result<Self, Self::Err> {
    match value.to_lowercase().as_str() {
      "en" => Ok(EN),
      "de" => Ok(DE),
      "ja" => Ok(JA),
      "es" => Ok(ES),
      _ => Err(Box::from(format!("Unknown language, {value}")))
    }
  }
}
