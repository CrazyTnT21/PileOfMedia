use crate::entities::account::{Email, Password};
use crate::entities::user::create_user::CreateUser;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreateAccount {
  pub user: CreateUser,
  pub email: Email,
  pub password: Password,
}
