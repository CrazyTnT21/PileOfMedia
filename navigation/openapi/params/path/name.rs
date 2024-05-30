#[derive(utoipa::IntoParams)]
#[into_params(names("name"))]
pub struct NameParam(
  ///Name of the item to search for
  String
);
