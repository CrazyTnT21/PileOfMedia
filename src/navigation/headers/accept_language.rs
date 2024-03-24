use std::convert::Infallible;
use std::str::FromStr;

use axum::async_trait;
use axum::extract::FromRequestParts;
use axum::http::header::ACCEPT_LANGUAGE;
use axum::http::request::Parts;

#[derive(Debug)]
pub enum AcceptLanguageError {
  MissingValue,
  InvalidQuality,
  MoreThanOneValue,
}

#[derive(Debug)]
pub struct AcceptLanguage {
  pub value: String,
  pub quality: Option<f32>,
}

impl FromStr for AcceptLanguage {
  type Err = AcceptLanguageError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut split_value = s.trim().split(';');

    let (value, quality) = (split_value.next(), split_value.next());
    let Some(value) = value else {
      return Err(AcceptLanguageError::MissingValue);
    };

    if value.is_empty() {
      return Err(AcceptLanguageError::MissingValue);
    }

    if let Some(_) = split_value.next() {
      return Err(AcceptLanguageError::MoreThanOneValue);
    }
    match quality.and_then(|q| q.strip_prefix("q=")) {
      None => Ok(AcceptLanguage { value: value.to_string(), quality: None }),
      Some(quality) => match quality.parse::<f32>() {
        Ok(quality) => Ok(AcceptLanguage { value: value.to_string(), quality: Some(quality) }),
        Err(_) => Err(AcceptLanguageError::InvalidQuality)
      }
    }
  }
}

#[derive(Debug)]
pub struct AcceptLanguageHeader(pub Vec<AcceptLanguage>);

impl Eq for AcceptLanguage {}

impl PartialEq for AcceptLanguage {
  fn eq(&self, other: &Self) -> bool {
    self.quality == other.quality && self.value.eq(&other.value)
  }
}

impl PartialOrd for AcceptLanguage {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for AcceptLanguage {
  fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    if self.quality > other.quality {
      std::cmp::Ordering::Greater
    } else if self.quality < other.quality {
      std::cmp::Ordering::Less
    } else {
      std::cmp::Ordering::Equal
    }
  }
}

impl FromStr for AcceptLanguageHeader {
  type Err = Infallible;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut value = s.trim().split(',');

    let mut values = Vec::new();
    while let Some(value) = value.next() {
      if let Ok(value) = AcceptLanguage::from_str(value) {
        values.push(value);
      }
    }

    Ok(AcceptLanguageHeader(values))
  }
}

#[async_trait]
impl<S> FromRequestParts<S> for AcceptLanguageHeader
  where
    S: Send + Sync,
{
  type Rejection = Infallible;

  async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
    Ok(parts.headers.get(ACCEPT_LANGUAGE)
      .and_then(|x| x.to_str().ok())
      .and_then(|x| AcceptLanguageHeader::from_str(x).ok())
      .unwrap_or(AcceptLanguageHeader(vec![])))
  }
}
