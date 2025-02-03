#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CharacterTranslation {
  pub name: String,
  pub first_name: Option<String>,
  pub last_name: Option<String>,
  pub description: Option<String>,
}
