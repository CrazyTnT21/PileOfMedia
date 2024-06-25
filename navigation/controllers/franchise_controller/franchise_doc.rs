use super::*;

#[derive(utoipa::OpenApi)]
#[openapi(tags((name = "Franchises", description = "Endpoints related to franchises")),
paths(get_items, get_by_id,get_by_name)
)]
pub(crate) struct FranchiseDoc;
