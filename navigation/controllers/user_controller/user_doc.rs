use super::*;

#[derive(utoipa::OpenApi)]
#[openapi(tags((name = "Users", description = "Endpoints related to users")),
paths(get_items, get_by_id,get_by_name)
)]
pub(crate) struct UserDoc;
