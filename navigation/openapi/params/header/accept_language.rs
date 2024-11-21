#[derive(utoipa::IntoParams, serde::Deserialize)]
#[into_params(names("Accept-Language"), parameter_in = Header)]
pub struct AcceptLanguageParam(
  ///The language of the items
  #[serde(default = "default_value")]
  Option<String>,
);

fn default_value() -> Option<String> {
  Some("en".to_string())
}
