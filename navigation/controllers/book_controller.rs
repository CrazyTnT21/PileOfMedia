use axum::{Json, Router};
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use bb8_postgres::bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::NoTls;
use domain::pagination::Pagination;
use crate::controllers::{content_language_header, convert_service_error, DEFAULT_LANGUAGE, get_book_service, get_language};
use crate::database_connection::DatabaseConnection;
use crate::headers::accept_language::AcceptLanguageHeader;

pub fn routes(pool: Pool<PostgresConnectionManager<NoTls>>) -> Router {
  Router::new()
    .route("/", get(get_items))
    .route("/:id", get(get_by_id))
    .route("/title/:title", get(get_by_title))
    .with_state(pool)
}

async fn get_items(AcceptLanguageHeader(languages): AcceptLanguageHeader, connection: DatabaseConnection) -> impl IntoResponse {
  let service = get_book_service(connection);
  let language = get_language(languages, DEFAULT_LANGUAGE);

  println!("Route for books in {}", language);

  let content_language = content_language_header(language);

  match service.get(language, Pagination::default()).await {
    Ok(books) => Ok((StatusCode::OK, content_language, Json(books))),
    Err(error) => Err(convert_service_error(error))
  }
}

async fn get_by_id(Path(id): Path<u32>, AcceptLanguageHeader(languages): AcceptLanguageHeader, connection: DatabaseConnection) -> impl IntoResponse {
  let service = get_book_service(connection);
  let language = get_language(languages, DEFAULT_LANGUAGE);

  println!("Route for a book with id {} in {}", id, language);

  let content_language = content_language_header(language);
  match service.get_by_id(id, language).await {
    Ok(item) => Ok((StatusCode::OK, content_language, Json(item))),
    Err(error) => Err(convert_service_error(error))
  }
}

async fn get_by_title(Path(title): Path<String>, AcceptLanguageHeader(languages): AcceptLanguageHeader, connection: DatabaseConnection) -> impl IntoResponse {
  let service = get_book_service(connection);
  let language = get_language(languages, DEFAULT_LANGUAGE);

  println!("Route for books with the title {} in {}", title, language);

  let content_language = content_language_header(language);
  match service.get_by_title(&title, language, Pagination::default()).await {
    Ok(items) => Ok((StatusCode::OK, content_language, Json(items))),
    Err(error) => Err(convert_service_error(error))
  }
}
