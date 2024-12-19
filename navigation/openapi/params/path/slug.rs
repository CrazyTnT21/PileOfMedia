use domain::slug::Slug;

#[derive(utoipa::IntoParams)]
#[into_params(names("slug"))]
pub struct SlugParam(
  ///Slug of the item to search for
  Slug,
);
