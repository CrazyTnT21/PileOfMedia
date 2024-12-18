use regex::Regex;
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct Slug(String);

#[cfg(feature = "serde")]
pub mod serde {
  use crate::slug::Slug;
  use serde::de::Error;
  use serde::{Deserializer, Serializer};

  impl serde::Serialize for Slug {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
      S: Serializer,
    {
      serializer.serialize_str(&self.0)
    }
  }
  impl<'de> serde::Deserialize<'de> for Slug {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
      D: Deserializer<'de>,
    {
      let value = String::deserialize(deserializer)?;
      let slug = Slug::parse(value).map_err(D::Error::custom)?;
      Ok(slug)
    }
  }
}

#[derive(Debug)]
pub enum SlugError {
  InvalidPattern(String),
}
impl Display for SlugError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}",
      match self {
        SlugError::InvalidPattern(value) => {
          format!("value '{value}' did not match pattern '{REGEX_PATTERN}'")
        }
      }
    )
  }
}
impl Error for SlugError {}
const REGEX_PATTERN: &str = r"^[a-z\-]+$";
impl Slug {
  /// # Panics
  ///
  /// Will panic if an error occurred during parsing or compiling the regular expression.
  pub fn parse(value: String) -> Result<Slug, SlugError> {
    let regex = Regex::new(REGEX_PATTERN).unwrap();
    let is_match = regex.is_match(&value);
    if !is_match {
      return Err(SlugError::InvalidPattern(value));
    }
    Ok(Slug(value))
  }
}
impl Display for Slug {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    String::fmt(&self.0, f)
  }
}
