use super::*;

#[derive(utoipa::OpenApi)]
#[openapi(tags((name = "Characters", description = "Endpoints related to characters")),
  paths(get_items, get_by_id, get_by_name, create_item, delete_item)
)]
pub(crate) struct CharacterDoc;
