use crate::entities::person::Person;
use crate::entities::role::Role;

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct PersonRole {
  pub person: Person,
  pub role: Role
}
