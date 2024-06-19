use super::*;

#[derive(utoipa::OpenApi)]
#[openapi(tags((name = "Books", description = "Endpoints related to books")),
  paths(
    get_items,
    get_by_id,
    get_by_title,
    get_genres,
    get_themes,
    get_characters,
    get_involved,
    add_character,
    remove_character,
    add_genre,
    remove_genre
  )
)]
pub(crate) struct BookDoc;
