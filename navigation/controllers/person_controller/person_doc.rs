use super::*;

#[derive(utoipa::OpenApi)]
#[openapi(tags((name = "People", description = "Endpoints related to people")),
  paths(get_items, get_by_id, get_by_name, create_item, delete_item)
)]
pub(crate) struct PersonDoc;
