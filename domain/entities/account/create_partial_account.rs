use crate::entities::account::{Email, Password};
use crate::entities::user::User;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreatePartialAccount {
  pub user: User,
  pub email: Email,
  pub password: Password,
}
