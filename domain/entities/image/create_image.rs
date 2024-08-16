#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreateImage(
  #[cfg_attr(feature = "serde", serde(skip))]
  pub Vec<u8>
);