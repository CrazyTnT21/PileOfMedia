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
  use std::error::Error;
  use std::fmt::{Display, Formatter};

  use crate::entities::person::create_person::{CreatePerson, CreatePersonData};
  use crate::entities::person::create_person::CreateImage;

  #[derive(Debug)]
  pub enum CreatePersonPart {
    Person,
    Image,
  }

  #[derive(Debug)]
  pub enum CreatePersonPartError {
    MissingPart,
    InvalidFormat,
    PersonMissing,
    UnknownPart(String),
    OtherError(Box<dyn Error + Send>),
  }

  impl Display for CreatePersonPartError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
      write!(f, "{}",
             match self {
               CreatePersonPartError::InvalidFormat => "Invalid part format".to_string(),
               CreatePersonPartError::UnknownPart(value) => format!("Unknown part '{}'", value),
               CreatePersonPartError::PersonMissing => "Person missing".to_string(),
               CreatePersonPartError::MissingPart => "Missing part value".to_string(),
               CreatePersonPartError::OtherError(value) => value.to_string(),
             })
    }
  }

  impl Error for CreatePersonPartError {}

  impl CreatePersonPart {
    fn from_header(value: &str) -> Result<Self, CreatePersonPartError> {
      if value.is_empty() {
        return Err(CreatePersonPartError::InvalidFormat);
      }
      let result = match value.to_lowercase().as_str() {
        "person" => CreatePersonPart::Person,
        "image" => CreatePersonPart::Image,
        _ => Err(CreatePersonPartError::UnknownPart(value.to_string()))?
      };
      Ok(result)
    }
  }

  #[async_trait::async_trait]
  impl multipart::FromMultiPart for CreatePerson {
    type Error = CreatePersonPartError;

    async fn from_multi_part(mut multipart: multipart::axum::extract::Multipart) -> Result<Self, Self::Error>
      where
        Self: Sized,
    {
      let mut person: Option<CreatePersonData> = None;
      let mut image = None;
      while let Some(a) = multipart.next_field().await.map_err(|x| CreatePersonPartError::OtherError(Box::new(x)))? {
        let part = CreatePersonPart::from_header(a.name().ok_or(CreatePersonPartError::MissingPart)?)?;
        match part {
          CreatePersonPart::Person => {
            let create_person = serde_json::from_slice::<CreatePersonData>(&a.bytes()
              .await
              .map_err(|x| CreatePersonPartError::OtherError(Box::new(x)))?)
              .map_err(|x| CreatePersonPartError::OtherError(Box::new(x)))?;
            person = Some(create_person);
          }
          CreatePersonPart::Image => { image = Some(a.bytes().await.map_err(|x| CreatePersonPartError::OtherError(Box::new(x)))?); }
        }
      }
      let data = person.ok_or(CreatePersonPartError::PersonMissing)?;
      let image = image.map(|x| CreateImage { 0: x.to_vec() });

      let user = CreatePerson { person: data, image };
      Ok(user)
    }
  }
}
