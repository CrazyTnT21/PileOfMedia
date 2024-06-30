use super::*;

#[derive(utoipa::OpenApi)]
#[openapi(tags((name = "Roles", description = "Endpoints related to roles")),
  paths(get_items, get_by_id, get_by_name, create_item, delete_item)
)]
pub(crate) struct RoleDoc;
