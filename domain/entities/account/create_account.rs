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
pub mod create_account_part {
  use crate::entities::account::create_account::{CreateAccount, CreateAccountData};
  use crate::entities::image::create_image::CreateImage;
  use crate::vec_single::{Single, SingleVecError};
  use multipart::axum::extract::multipart::MultipartError;
  use multipart::serialize_parts;
  use serde_json::from_slice;
  use std::error::Error;
  use std::fmt::{Display, Formatter};

  #[derive(Debug)]
  pub enum CreateAccountPart {
    Account,
    ProfilePicture,
  }

  #[derive(Debug)]
  pub enum CreateAccountPartError {
    AccountMissing,
    MoreThanOneAccount,
    MoreThanOneProfilePicture,
    OtherError(Box<dyn Error + Send>),
  }

  impl Display for CreateAccountPartError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
      write!(
        f,
        "{}",
        match self {
          CreateAccountPartError::AccountMissing => "Account missing".to_string(),
          CreateAccountPartError::OtherError(value) => value.to_string(),
          CreateAccountPartError::MoreThanOneAccount => "There is more than 1 account".to_string(),
          CreateAccountPartError::MoreThanOneProfilePicture => "There is more than 1 profile picture".to_string(),
        }
      )
    }
  }

  impl Error for CreateAccountPartError {}

  impl From<serde_json::Error> for CreateAccountPartError {
    fn from(value: serde_json::Error) -> Self {
      CreateAccountPartError::OtherError(Box::new(value))
    }
  }
  impl From<MultipartError> for CreateAccountPartError {
    fn from(value: MultipartError) -> Self {
      CreateAccountPartError::OtherError(Box::new(value))
    }
  }
  impl multipart::FromMultiPart for CreateAccount {
    type Error = CreateAccountPartError;

    async fn from_multi_part(multipart: multipart::axum::extract::Multipart) -> Result<Self, Self::Error>
    where
      Self: Sized,
    {
      let mut parts = serialize_parts(multipart).await?;
      let account_bytes = parts
        .remove(&Some("account".to_string()))
        .ok_or_else(|| CreateAccountPartError::AccountMissing)?
        .single()
        .map_err(|x| match x {
          SingleVecError::NoItems => CreateAccountPartError::AccountMissing,
          SingleVecError::MoreThanOneItem(_) => CreateAccountPartError::MoreThanOneAccount,
        })?;

      let account: CreateAccountData = from_slice(&account_bytes)?;

      let profile_picture = parts
        .remove(&Some("profile_picture".to_string()))
        .unwrap_or_else(Vec::new);
      let profile_picture = profile_picture
        .into_iter()
        .map(|x| CreateImage(x.to_vec()))
        .collect::<Vec<CreateImage>>()
        .single();
      let profile_picture = match profile_picture {
        Ok(pic) => Ok(Some(pic)),
        Err(err) => match err {
          SingleVecError::NoItems => Ok(None),
          SingleVecError::MoreThanOneItem(_) => Err(CreateAccountPartError::MoreThanOneProfilePicture),
        },
      }?;

      Ok(CreateAccount {
        account,
        profile_picture,
      })
    }
  }
}
