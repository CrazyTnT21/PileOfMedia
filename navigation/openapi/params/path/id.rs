#[derive(utoipa::IntoParams)]
#[into_params(names("id"))]
pub struct IdParam(
  ///Id of the item to search for
  u32,
);
