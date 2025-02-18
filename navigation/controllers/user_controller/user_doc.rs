use super::*;

#[derive(utoipa::OpenApi)]
#[openapi(tags((name = "Users", description = "Endpoints related to users")),
paths(get_items, get_by_id,get_by_name,get_books,get_book_by_id)
)]
pub(crate) struct UserDoc;
