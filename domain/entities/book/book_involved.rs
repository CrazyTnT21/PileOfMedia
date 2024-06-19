use crate::entities::person::Person;
use crate::entities::person::person_role::PersonRole;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct BookInvolved {
  pub person: Person,
  pub role: PersonRole,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct InvolvedId {
  pub person_id: u32,
  pub role_id: u32,
}
/* TODO:
pub struct BookInvolved {
  pub person: Person,
  pub roles: Vec<PersonRole>,
}
*/
