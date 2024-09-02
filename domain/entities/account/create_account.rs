use crate::entities::account::{Email, Password};
use crate::entities::image::create_image::CreateImage;
use crate::entities::user::create_user::CreateUserData;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreateAccount {
  pub account: CreateAccountData,
  pub profile_picture: Option<CreateImage>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreateAccountData {
  pub user: CreateUserData,
  pub email: Email,
  pub password: Password,
}

#[cfg(feature = "axum-multipart")]
pub mod create_book_part {
  use std::error::Error;
  use std::fmt::{Display, Formatter};

  use crate::entities::account::create_account::{CreateAccount, CreateAccountData};
  use crate::entities::image::create_image::CreateImage;

  #[derive(Debug)]
  pub enum CreateAccountPart {
    Account,
    ProfilePicture,
  }

  #[derive(Debug)]
  pub enum CreateAccountPartError {
    MissingPart,
    InvalidFormat,
    AccountMissing,
    UnknownPart(String),
    OtherError(Box<dyn Error + Send>),
  }

  impl Display for CreateAccountPartError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
      write!(f, "{}",
             match self {
               CreateAccountPartError::InvalidFormat => "Invalid part format".to_string(),
               CreateAccountPartError::UnknownPart(value) => format!("Unknown part '{}'", value),
               CreateAccountPartError::AccountMissing => "Account missing".to_string(),
               CreateAccountPartError::MissingPart => "Missing part value".to_string(),
               CreateAccountPartError::OtherError(value) => value.to_string(),
             })
    }
  }

  impl Error for CreateAccountPartError {}

  impl CreateAccountPart {
    fn from_header(value: &str) -> Result<Self, CreateAccountPartError> {
      if value.is_empty() {
        return Err(CreateAccountPartError::InvalidFormat);
      }
      let result = match value.to_lowercase().as_str() {
        "account" => CreateAccountPart::Account,
        "profile_picture" => CreateAccountPart::ProfilePicture,
        _ => Err(CreateAccountPartError::UnknownPart(value.to_string()))?
      };
      Ok(result)
    }
  }

  #[async_trait::async_trait]
  impl multipart::FromMultiPart for CreateAccount {
    type Error = CreateAccountPartError;

    async fn from_multi_part(mut multipart: multipart::axum::extract::Multipart) -> Result<Self, Self::Error>
      where
        Self: Sized,
    {
      let mut data: Option<CreateAccountData> = None;
      let mut image = None;
      while let Some(a) = multipart.next_field().await.map_err(|x| CreateAccountPartError::OtherError(Box::new(x)))? {
        let part = CreateAccountPart::from_header(a.name().ok_or(CreateAccountPartError::MissingPart)?)?;
        match part {
          CreateAccountPart::Account => {
            let create_account = serde_json::from_slice::<CreateAccountData>(&a.bytes()
              .await
              .map_err(|x| CreateAccountPartError::OtherError(Box::new(x)))?)
              .map_err(|x| CreateAccountPartError::OtherError(Box::new(x)))?;
            data = Some(create_account);
          }
          CreateAccountPart::ProfilePicture => { image = Some(a.bytes().await.map_err(|x| CreateAccountPartError::OtherError(Box::new(x)))?); }
        }
      }
      let data = data.ok_or(CreateAccountPartError::AccountMissing)?;
      let profile_picture = image.map(|x| CreateImage { 0: x.to_vec() });

      let account = CreateAccount { account: data, profile_picture };
      Ok(account)
    }
  }
}
