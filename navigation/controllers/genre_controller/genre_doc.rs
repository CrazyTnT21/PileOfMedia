use super::*;

#[derive(utoipa::OpenApi)]
#[openapi(tags((name = "Genres", description = "Endpoints related to genres")),
paths(get_items, get_by_id,get_by_name)
)]
pub(crate) struct GenreDoc;
