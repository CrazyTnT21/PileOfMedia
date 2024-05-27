use axum::{Json, Router};
use axum::extract::{Path, Query};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use bb8_postgres::bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::NoTls;

use domain::entities::book::Book;
use domain::entities::franchise::Franchise;
use domain::entities::image::Image;
use domain::entities::image::image_data::ImageData;
use domain::items_total::BooksTotal;
use services::book_service::BookService;

use crate::controllers::{content_language_header, convert_service_error, DEFAULT_LANGUAGE, get_book_repository, get_book_service, get_image_repository, get_language, append_content_language_header, set_pagination_limit};
use crate::database_connection::DatabaseConnection;
use crate::extractors::headers::accept_language::AcceptLanguageHeader;
use crate::extractors::query_pagination::QueryPagination;
use crate::openapi::params::header::accept_language::AcceptLanguageParam;
use crate::openapi::params::path::id::IdParam;
use crate::openapi::params::path::title::TitleParam;
use crate::openapi::params::query::count::CountParam;
use crate::openapi::params::query::page::PageParam;
use crate::openapi::responses::bad_request::BadRequest;
use crate::openapi::responses::not_found::NotFound;
use crate::openapi::responses::server_error::ServerError;

#[derive(utoipa::OpenApi)]
#[openapi(tags((name = "Books", description = "Endpoints related to books")),
paths(get_items, get_by_id, get_by_title),
components(schemas(Book, Image, ImageData, Franchise, BooksTotal)))]
pub(crate) struct BookDoc;

pub fn routes(pool: Pool<PostgresConnectionManager<NoTls>>) -> Router {
  Router::new()
    .route("/", get(get_items))
    .route("/:id", get(get_by_id))
    .route("/title/:title", get(get_by_title))
    .with_state(pool)
}

#[utoipa::path(get, path = "",
responses(
(status = 200, description = "Returned books", body = BooksTotal), ServerError, BadRequest),
params(AcceptLanguageParam, PageParam, CountParam),
tag = "Books"
)]
async fn get_items(AcceptLanguageHeader(languages): AcceptLanguageHeader, connection: DatabaseConnection, Query(mut pagination): Query<QueryPagination>) -> impl IntoResponse {
  let connection = connection.0;
  let image_repository = get_image_repository(&connection);
  let repository = get_book_repository(&connection, &image_repository);
  let service = get_book_service(&repository);

  let language = get_language(languages, DEFAULT_LANGUAGE);
  set_pagination_limit(&mut pagination);

  println!("Route for books in {}", language);

  let mut content_language = content_language_header(language);
  append_content_language_header(&mut content_language, DEFAULT_LANGUAGE);

  match service.get(language, pagination.into()).await {
    Ok(books) => Ok((StatusCode::OK, content_language, Json(books))),
    Err(error) => Err(convert_service_error(error))
  }
}

#[utoipa::path(get, path = "/{id}",
responses(
(status = 200, description = "Returned book based on the id", body = Book), ServerError, BadRequest, NotFound),
params(IdParam, AcceptLanguageParam),
tag = "Books"
)]
async fn get_by_id(Path(id): Path<u32>, AcceptLanguageHeader(languages): AcceptLanguageHeader, connection: DatabaseConnection) -> impl IntoResponse {
  let connection = connection.0;
  let image_repository = get_image_repository(&connection);
  let repository = get_book_repository(&connection, &image_repository);
  let service = get_book_service(&repository);

  let language = get_language(languages, DEFAULT_LANGUAGE);

  println!("Route for a book with id {} in {}", id, language);

  let mut content_language = content_language_header(language);
  append_content_language_header(&mut content_language, DEFAULT_LANGUAGE);

  match service.get_by_id(id, language).await {
    Ok(item) => Ok((StatusCode::OK, content_language, Json(item))),
    Err(error) => Err(convert_service_error(error))
  }
}

#[utoipa::path(get, path = "/title/{title}",
responses(
(status = 200, description = "Returned books based on the title", body = BooksTotal), ServerError, BadRequest),
params(TitleParam, AcceptLanguageParam, PageParam, CountParam),
tag = "Books"
)]
async fn get_by_title(Path(title): Path<String>, AcceptLanguageHeader(languages): AcceptLanguageHeader, connection: DatabaseConnection, Query(mut pagination): Query<QueryPagination>) -> impl IntoResponse {
  let connection = connection.0;
  let image_repository = get_image_repository(&connection);
  let repository = get_book_repository(&connection, &image_repository);
  let service = get_book_service(&repository);

  let language = get_language(languages, DEFAULT_LANGUAGE);
  set_pagination_limit(&mut pagination);

  println!("Route for books with the title {} in {}", title, language);

  let mut content_language = content_language_header(language);
  append_content_language_header(&mut content_language, DEFAULT_LANGUAGE);

  match service.get_by_title(&title, language, pagination.into()).await {
    Ok(items) => Ok((StatusCode::OK, content_language, Json(items))),
    Err(error) => Err(convert_service_error(error))
  }
}
