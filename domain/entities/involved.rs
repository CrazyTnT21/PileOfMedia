use crate::entities::person::person_role::PersonRole;
use crate::entities::person::Person;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct Involved {
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

impl Display for InvolvedId {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{{ Person: {}, Role: {} }}", self.person_id, self.role_id)
  }
}
