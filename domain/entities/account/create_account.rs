use crate::entities::account::{Email, Password};
use crate::entities::user::User;

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreateAccount {
  pub user: User,
  pub email: Email,
  pub password: Password,
}
