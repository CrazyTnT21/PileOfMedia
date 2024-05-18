#[derive(utoipa::IntoParams, serde::Deserialize)]
#[into_params(names("count"), parameter_in = Query)]
pub struct CountParam(
  ///The amount of items to query
  #[param(minimum = 0, maximum = 50)]
  #[serde(default = "default_value")]
  Option<u32>
);

fn default_value() -> Option<u32> { Some(50) }
