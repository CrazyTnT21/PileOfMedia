use std::fmt::{Debug, Display, Formatter};

#[derive(Clone, Debug)]
pub struct Score<Number = u8, const MIN: usize = 1, const MAX: usize = 10>(Number);

#[cfg(feature = "utoipa")]
impl<'a, Number, const MIN: usize, const MAX: usize> utoipa::ToSchema<'a> for Score<Number, MIN, MAX> {
  fn schema() -> (&'a str, utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>) {
    let mut object = utoipa::openapi::Object::new();
    object.schema_type = utoipa::openapi::SchemaType::Integer;
    object.nullable = false;
    object.minimum = Some(1f64);
    object.maximum = Some(10f64);

    ("Score", object.into())
  }
}
impl Score<f32> {
  pub const fn to_f32(self) -> f32 {
    self.0
  }
}

impl Score<u8> {
  pub const fn to_u8(self) -> u8 {
    self.0
  }
}

impl<const MIN: usize, const MAX: usize> TryFrom<u8> for Score<u8, MIN, MAX> {
  type Error = ScoreError<u8>;

  fn try_from(value: u8) -> Result<Self, Self::Error> {
    let value_usize = usize::from(value);
    if value_usize < MIN || value_usize > MAX {
      return Err(ScoreError::NotInScale {
        value,
        min: MIN,
        max: MAX,
      });
    }
    Ok(Score(value))
  }
}

impl<const MIN: usize, const MAX: usize> TryFrom<u64> for Score<u8, MIN, MAX> {
  type Error = ScoreError<u8>;

  fn try_from(value: u64) -> Result<Self, Self::Error> {
    let value = value.try_into().map_err(|_| ScoreError::InvalidCast)?;
    let value_usize = usize::from(value);
    if value_usize < MIN || value_usize > MAX {
      return Err(ScoreError::NotInScale {
        value,
        min: MIN,
        max: MAX,
      });
    }
    Ok(Score(value))
  }
}
impl<const MIN: usize, const MAX: usize> TryFrom<u32> for Score<u8, MIN, MAX> {
  type Error = ScoreError<u8>;

  fn try_from(value: u32) -> Result<Self, Self::Error> {
    let value = value.try_into().map_err(|_| ScoreError::InvalidCast)?;
    let value_usize = usize::from(value);
    if value_usize < MIN || value_usize > MAX {
      return Err(ScoreError::NotInScale {
        value,
        min: MIN,
        max: MAX,
      });
    }
    Ok(Score(value))
  }
}

impl<const MIN: usize, const MAX: usize> TryFrom<u16> for Score<u8, MIN, MAX> {
  type Error = ScoreError<u8>;

  fn try_from(value: u16) -> Result<Self, Self::Error> {
    let value = value.try_into().map_err(|_| ScoreError::InvalidCast)?;
    let value_usize = usize::from(value);
    if value_usize < MIN || value_usize > MAX {
      return Err(ScoreError::NotInScale {
        value,
        min: MIN,
        max: MAX,
      });
    }
    Ok(Score(value))
  }
}

impl<const MIN: usize, const MAX: usize> TryFrom<f32> for Score<f32, MIN, MAX> {
  type Error = ScoreError<f32>;

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
impl<const MIN: usize, const MAX: usize> TryFrom<i64> for Score<f32, MIN, MAX> {
  type Error = ScoreError<f32>;

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

impl<const MIN: usize, const MAX: usize> TryFrom<u64> for Score<f32, MIN, MAX> {
  type Error = ScoreError<f32>;

  fn try_from(value: u64) -> Result<Self, Self::Error> {
    let value_usize: usize = value.try_into().map_err(|_| ScoreError::InvalidCast)?;

    #[allow(clippy::cast_possible_truncation)]
    let value = value as f32;
    if value_usize < MIN || value_usize > MAX {
      return Err(ScoreError::NotInScale {
        value,
        min: MIN,
        max: MAX,
      });
    }
    Ok(Score(value))
  }
}
impl<const MIN: usize, const MAX: usize> TryFrom<u32> for Score<f32, MIN, MAX> {
  type Error = ScoreError<f32>;

  fn try_from(value: u32) -> Result<Self, Self::Error> {
    let value_usize: usize = value.try_into().map_err(|_| ScoreError::InvalidCast)?;

    #[allow(clippy::cast_possible_truncation)]
    let value = value as f32;
    if value_usize < MIN || value_usize > MAX {
      return Err(ScoreError::NotInScale {
        value,
        min: MIN,
        max: MAX,
      });
    }
    Ok(Score(value))
  }
}

impl<const MIN: usize, const MAX: usize> TryFrom<u16> for Score<f32, MIN, MAX> {
  type Error = ScoreError<f32>;

  fn try_from(value: u16) -> Result<Self, Self::Error> {
    let value_usize: usize = value.into();
    let value = value.into();
    if value_usize < MIN || value_usize > MAX {
      return Err(ScoreError::NotInScale {
        value,
        min: MIN,
        max: MAX,
      });
    }
    Ok(Score(value))
  }
}
#[derive(Debug)]
pub enum ScoreError<Number> {
  NotInScale { value: Number, min: usize, max: usize },
  InvalidCast,
}

impl<Number: Display> Display for ScoreError<Number> {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}",
      match self {
        ScoreError::NotInScale { value, min, max } => format!("Value '{value}' not between {min} - {max}"),
        ScoreError::InvalidCast => "Value could not be cast into the given type".to_string(),
      }
    )
  }
}

impl<Number: Display + Debug> std::error::Error for ScoreError<Number> {}

#[cfg(feature = "serde")]
pub mod serde {
  use crate::score::Score;
  use serde::de::Error;
  use serde::{Deserialize, Deserializer, Serialize, Serializer};
  use std::fmt::Display;

  impl<Number: Serialize, const MIN: usize, const MAX: usize> serde::Serialize for Score<Number, MIN, MAX> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
      S: Serializer,
    {
      <Number as Serialize>::serialize(&self.0, serializer)
    }
  }
  impl<
    'de,
    Number: Deserialize<'de> + TryInto<Score<Number, MIN, MAX>, Error: Display>,
    const MIN: usize,
    const MAX: usize,
  > serde::Deserialize<'de> for Score<Number, MIN, MAX>
  {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
      D: Deserializer<'de>,
    {
      let value = <Number as Deserialize>::deserialize(deserializer).map_err(D::Error::custom)?;
      let score = Number::try_into(value).map_err(D::Error::custom)?;
      Ok(score)
    }
  }
}
