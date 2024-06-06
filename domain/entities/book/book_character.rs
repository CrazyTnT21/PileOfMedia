use crate::entities::character::Character;

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct BookCharacter {
  pub character: Character
}
