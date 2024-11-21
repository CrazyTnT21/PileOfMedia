#[derive(utoipa::IntoParams, serde::Deserialize)]
#[into_params(names("page"), parameter_in = Query)]
pub struct PageParam(
  ///The current page
  #[param(minimum = 0)]
  #[serde(default)]
  Option<u32>,
);
