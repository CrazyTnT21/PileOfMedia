use super::*;

#[derive(utoipa::OpenApi)]
#[openapi(tags((name = "Books", description = "Endpoints related to books")),
paths(get_items, get_by_id, get_by_title)
)]
pub(crate) struct BookDoc;
