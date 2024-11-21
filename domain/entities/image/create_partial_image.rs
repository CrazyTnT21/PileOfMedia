#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreatePartialImage<'a> {
  pub file_name: &'a str,
  pub file_path: &'a str,
  pub uri: &'a str,
  pub display_path: &'a str,
}
