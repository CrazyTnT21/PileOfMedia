use std::sync::Arc;

use axum::{Json, Router};
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;

use crate::application::pagination::Pagination;
use crate::get_book_service;
use crate::navigation::controllers::{content_language_header, convert_service_error, DEFAULT_LANGUAGE, get_language};
use crate::navigation::headers::accept_language::AcceptLanguageHeader;
use crate::services::book_service::BookService;

pub fn add_routes(router: Router) -> Router {
  router
    .nest("/books",
          Router::new()
            .route("/", get(get_items))
            .route("/:id", get(get_by_id))
            .route("/title/:title", get(get_by_title))
            .with_state(get_book_service()),
    )
}

async fn get_items(AcceptLanguageHeader(languages): AcceptLanguageHeader, State(book_service): State<Arc<dyn BookService>>) -> impl IntoResponse {
  let language = get_language(languages, DEFAULT_LANGUAGE);

  println!("Route for books in {}", language);

  let content_language = content_language_header(language);

  match book_service.get(language, Pagination::default()) {
    Ok(books) => Ok((StatusCode::OK, content_language, Json(books))),
    Err(error) => Err(convert_service_error(error))
  }
}

async fn get_by_id(Path(id): Path<u32>, AcceptLanguageHeader(languages): AcceptLanguageHeader, State(book_service): State<Arc<dyn BookService>>) -> impl IntoResponse {
  let language = get_language(languages, DEFAULT_LANGUAGE);

  println!("Route for a book with id {} in {}", id, language);

  let content_language = content_language_header(language);
  match book_service.get_by_id(id, language) {
    Ok(item) => Ok((StatusCode::OK, content_language, Json(item))),
    Err(error) => Err(convert_service_error(error))
  }
}

async fn get_by_title(Path(title): Path<String>, AcceptLanguageHeader(languages): AcceptLanguageHeader, State(book_service): State<Arc<dyn BookService>>) -> impl IntoResponse {
  let language = get_language(languages, DEFAULT_LANGUAGE);

  println!("Route for books with the title {} in {}", title, language);

  let content_language = content_language_header(language);
  match book_service.get_by_title(&title, language, Pagination::default()) {
    Ok(items) => Ok((StatusCode::OK, content_language, Json(items))),
    Err(error) => Err(convert_service_error(error))
  }
}
