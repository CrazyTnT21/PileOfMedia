use crate::entities::account::{Email, Password};
use crate::entities::user::partial_create_user::PartialCreateUser;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct PartialCreateAccount {
  pub user: PartialCreateUser,
  pub email: Email,
  pub password: Password,
}
