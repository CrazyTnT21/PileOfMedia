#[derive(utoipa::IntoParams, serde::Deserialize)]
#[into_params(names("Authorization"), parameter_in = Header)]
//TODO: Fix
pub struct JsonWebTokenParam(
  ///JWT
  String
);
