use super::*;

#[derive(utoipa::OpenApi)]
#[openapi(tags((name = "Themes", description = "Endpoints related to themes")),
  paths(get_items, get_by_id, get_by_name, create_item, delete_item)
)]
pub(crate) struct ThemeDoc;
