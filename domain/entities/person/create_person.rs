use std::collections::HashMap;

use chrono::NaiveDate;

use crate::entities::image::create_image::CreateImage;
use crate::enums::language::Language;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreatePerson {
  pub person: CreatePersonData,
  pub image: Option<CreateImage>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreatePersonData {
  pub name: String,
  pub first_name: Option<String>,
  pub last_name: Option<String>,
  pub birthday: Option<NaiveDate>,
  pub height: Option<u16>,
  pub translations: HashMap<Language, CreatePersonTranslation>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreatePersonTranslation {
  pub description: Option<String>,
}

#[cfg(feature = "axum-multipart")]
pub mod create_person_part {
  use crate::entities::person::create_person::CreateImage;
  use crate::entities::person::create_person::{CreatePerson, CreatePersonData};
  use crate::vec_single::{Single, SingleVecError};
  use multipart::axum::extract::multipart::MultipartError;
  use multipart::serialize_parts;
  use serde_json::from_slice;
  use std::error::Error;
  use std::fmt::{Display, Formatter};

  #[derive(Debug)]
  pub enum CreatePersonPart {
    Person,
    Image,
  }

  #[derive(Debug)]
  pub enum CreatePersonPartError {
    PersonMissing,
    MoreThanOnePerson,
    MoreThanOneProfilePicture,
    OtherError(Box<dyn Error + Send>),
  }

  impl Display for CreatePersonPartError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
      write!(
        f,
        "{}",
        match self {
          CreatePersonPartError::PersonMissing => "Person missing".to_string(),
          CreatePersonPartError::OtherError(value) => value.to_string(),
          CreatePersonPartError::MoreThanOnePerson => "There is more than 1 person".to_string(),
          CreatePersonPartError::MoreThanOneProfilePicture => "There is more than 1 profile picture".to_string(),
        }
      )
    }
  }
  impl Error for CreatePersonPartError {}
  impl From<serde_json::Error> for CreatePersonPartError {
    fn from(value: serde_json::Error) -> Self {
      CreatePersonPartError::OtherError(Box::new(value))
    }
  }
  impl From<MultipartError> for CreatePersonPartError {
    fn from(value: MultipartError) -> Self {
      CreatePersonPartError::OtherError(Box::new(value))
    }
  }

  #[async_trait::async_trait]
  impl multipart::FromMultiPart for CreatePerson {
    type Error = CreatePersonPartError;

    async fn from_multi_part(multipart: multipart::axum::extract::Multipart) -> Result<Self, Self::Error>
    where
      Self: Sized,
    {
      let mut parts = serialize_parts(multipart).await?;
      let person_bytes = parts
        .remove(&Some("person".to_string()))
        .ok_or_else(|| CreatePersonPartError::PersonMissing)?
        .single()
        .map_err(|x| match x {
          SingleVecError::NoItems => CreatePersonPartError::PersonMissing,
          SingleVecError::MoreThanOneItem(_) => CreatePersonPartError::MoreThanOnePerson,
        })?;

      let person: CreatePersonData = from_slice(&person_bytes)?;

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
          SingleVecError::MoreThanOneItem(_) => Err(CreatePersonPartError::MoreThanOneProfilePicture),
        },
      }?;

      Ok(CreatePerson { person, image })
    }
  }
}
