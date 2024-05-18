#[derive(utoipa::IntoParams)]
#[into_params(names("title"))]
pub struct TitleParam(
  ///Title of the item to search for
  String
);
