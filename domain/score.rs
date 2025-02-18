use std::fmt::{Display, Formatter};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct Score<const MIN: usize = 1, const MAX: usize = 10>(f32);

impl Score {
  pub const fn to_f32(self) -> f32 {
    self.0
  }
}

impl<const MIN: usize, const MAX: usize> TryFrom<f32> for Score<MIN, MAX> {
  type Error = ScoreError;

  fn try_from(value: f32) -> Result<Self, Self::Error> {
    if value < MIN as f32 || value > MAX as f32 {
      return Err(ScoreError::NotInScale {
        value,
        min: MIN,
        max: MAX,
      });
    }
    Ok(Score(value))
  }
}
impl<const MIN: usize, const MAX: usize> TryFrom<i64> for Score<MIN, MAX> {
  type Error = ScoreError;

  fn try_from(value: i64) -> Result<Self, Self::Error> {
    if value < MIN as i64 || value > MAX as i64 {
      return Err(ScoreError::NotInScale {
        value: value as f32,
        min: MIN,
        max: MAX,
      });
    }
    Ok(Score(value as f32))
  }
}
impl<const MIN: usize, const MAX: usize> TryFrom<i32> for Score<MIN, MAX> {
  type Error = ScoreError;

  fn try_from(value: i32) -> Result<Self, Self::Error> {
    if value < MIN as i32 || value > MAX as i32 {
      return Err(ScoreError::NotInScale {
        value: value as f32,
        min: MIN,
        max: MAX,
      });
    }
    Ok(Score(value as f32))
  }
}

impl<const MIN: usize, const MAX: usize> TryFrom<i16> for Score<MIN, MAX> {
  type Error = ScoreError;

  fn try_from(value: i16) -> Result<Self, Self::Error> {
    if value < MIN as i16 || value > MAX as i16 {
      return Err(ScoreError::NotInScale {
        value: value.into(),
        min: MIN,
        max: MAX,
      });
    }
    Ok(Score(value.into()))
  }
}

impl<const MIN: usize, const MAX: usize> TryFrom<u64> for Score<MIN, MAX> {
  type Error = ScoreError;

  fn try_from(value: u64) -> Result<Self, Self::Error> {
    if value < MIN as u64 || value > MAX as u64 {
      return Err(ScoreError::NotInScale {
        value: value as f32,
        min: MIN,
        max: MAX,
      });
    }
    Ok(Score(value as f32))
  }
}
impl<const MIN: usize, const MAX: usize> TryFrom<u32> for Score<MIN, MAX> {
  type Error = ScoreError;

  fn try_from(value: u32) -> Result<Self, Self::Error> {
    if value < MIN as u32 || value > MAX as u32 {
      return Err(ScoreError::NotInScale {
        value: value as f32,
        min: MIN,
        max: MAX,
      });
    }
    Ok(Score(value as f32))
  }
}

impl<const MIN: usize, const MAX: usize> TryFrom<u16> for Score<MIN, MAX> {
  type Error = ScoreError;

  fn try_from(value: u16) -> Result<Self, Self::Error> {
    if value < MIN as u16 || value > MAX as u16 {
      return Err(ScoreError::NotInScale {
        value: value.into(),
        min: MIN,
        max: MAX,
      });
    }
    Ok(Score(value.into()))
  }
}
#[derive(Debug)]
pub enum ScoreError {
  NotInScale { value: f32, min: usize, max: usize },
}

impl Display for ScoreError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}",
      match self {
        ScoreError::NotInScale { value, min, max } => format!("Value '{value}' not between {min} - {max}"),
      }
    )
  }
}

impl std::error::Error for ScoreError {}

#[cfg(feature = "serde")]
pub mod serde {
  use crate::score::Score;
  use serde::de::Error;
  use serde::{Deserializer, Serializer};

  impl<const MIN: usize, const MAX: usize> serde::Serialize for Score<MIN, MAX> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
      S: Serializer,
    {
      serializer.serialize_f32(self.0)
    }
  }
  impl<'de, const MIN: usize, const MAX: usize> serde::Deserialize<'de> for Score<MIN, MAX> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
      D: Deserializer<'de>,
    {
      let value = f32::deserialize(deserializer)?;
      let score = Score::try_from(value).map_err(D::Error::custom)?;
      Ok(score)
    }
  }
}
