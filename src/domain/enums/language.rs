use std::fmt::{Display, Formatter};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Language {
  EN,
  DE,
  JA,
  ES,
}

impl Display for Language {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", match self {
      Language::EN => "English",
      Language::DE => "Deutsch",
      Language::JA => "日本語",
      Language::ES => "Español"
    })
  }
}
