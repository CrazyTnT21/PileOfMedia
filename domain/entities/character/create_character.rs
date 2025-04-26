use crate::entities::image::create_image::CreateImage;
use crate::enums::language::Language;
use chrono::NaiveDate;
use std::collections::HashMap;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreateCharacter {
  pub character: CreateCharacterData,
  pub image: Option<CreateImage>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreateCharacterData {
  pub birthday: Option<NaiveDate>,
  pub height_cm: Option<u32>,
  pub translations: HashMap<Language, CreateCharacterTranslation>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreateCharacterTranslation {
  pub name: String,
  pub first_name: Option<String>,
  pub last_name: Option<String>,
  pub description: Option<String>,
}

#[cfg(feature = "axum-multipart")]
pub mod create_character_part {
  use crate::entities::character::create_character::CreateImage;
  use crate::entities::character::create_character::{CreateCharacter, CreateCharacterData};
  use crate::vec_single::{Single, SingleVecError};
  use multipart::axum::extract::multipart::MultipartError;
  use multipart::serialize_parts;
  use serde_json::from_slice;
  use std::error::Error;
  use std::fmt::{Display, Formatter};

  #[derive(Debug)]
  pub enum CreateCharacterPart {
    Character,
    Image,
  }

  #[derive(Debug)]
  pub enum CreateCharacterPartError {
    CharacterMissing,
    MoreThanOneCharacter,
    MoreThanOneImage,
    OtherError(Box<dyn Error + Send>),
  }

  impl Display for CreateCharacterPartError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
      write!(
        f,
        "{}",
        match self {
          CreateCharacterPartError::CharacterMissing => "Character missing".to_string(),
          CreateCharacterPartError::OtherError(value) => value.to_string(),
          CreateCharacterPartError::MoreThanOneCharacter => "There is more than 1 character".to_string(),
          CreateCharacterPartError::MoreThanOneImage => "There is more than 1 profile picture".to_string(),
        }
      )
    }
  }
  impl Error for CreateCharacterPartError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
      match self {
        CreateCharacterPartError::OtherError(e) => Some(&**e),
        _ => None,
      }
    }
  }
  impl From<serde_json::Error> for CreateCharacterPartError {
    fn from(value: serde_json::Error) -> Self {
      CreateCharacterPartError::OtherError(Box::new(value))
    }
  }
  impl From<MultipartError> for CreateCharacterPartError {
    fn from(value: MultipartError) -> Self {
      CreateCharacterPartError::OtherError(Box::new(value))
    }
  }

  impl multipart::FromMultiPart for CreateCharacter {
    type Error = CreateCharacterPartError;

    async fn from_multi_part(multipart: multipart::axum::extract::Multipart) -> Result<Self, Self::Error>
    where
      Self: Sized,
    {
      let mut parts = serialize_parts(multipart).await?;
      let character_bytes = parts
        .remove(&Some("character".to_string()))
        .ok_or_else(|| CreateCharacterPartError::CharacterMissing)?
        .single()
        .map_err(|x| match x {
          SingleVecError::NoItems => CreateCharacterPartError::CharacterMissing,
          SingleVecError::MoreThanOneItem(_) => CreateCharacterPartError::MoreThanOneCharacter,
        })?;

      let character: CreateCharacterData = from_slice(&character_bytes)?;

      let image = parts.remove(&Some("image".to_string())).unwrap_or_else(Vec::new);
      let image = image
        .into_iter()
        .map(|x| CreateImage(x.to_vec()))
        .collect::<Vec<CreateImage>>()
        .single();
      let image = match image {
        Ok(pic) => Ok(Some(pic)),
        Err(err) => match err {
          SingleVecError::NoItems => Ok(None),
          SingleVecError::MoreThanOneItem(_) => Err(CreateCharacterPartError::MoreThanOneImage),
        },
      }?;

      Ok(CreateCharacter { character, image })
    }
  }
}
