pub mod create_account;
pub mod create_partial_account;

use crate::entities::user::User;

#[derive(Debug, Clone)]
pub struct Account {
  pub user: User,
  pub password: Password,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct Password(pub String);
