use std::str::FromStr;
use std::sync::Arc;

use axum::http::{HeaderMap, StatusCode};
use axum::Router;
use bb8_postgres::bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::NoTls;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use application::repositories::default_book_repository::DefaultBookRepository;
use domain::enums::language::Language;
use domain::enums::language::Language::EN;
use infrastructure::default_book_service::DefaultBookService;
use repositories::book_repository::BookRepository;
use services::book_service::BookService;
use services::traits::service_error::ServiceError;
use crate::controllers::book_controller::BookDoc;

use crate::database_connection::DatabaseConnection;
use crate::extractors::headers::accept_language::AcceptLanguage;
use crate::extractors::query_pagination::QueryPagination;

mod book_controller;

#[derive(utoipa::OpenApi)]
#[openapi(info(title = "mycollection"), nest(("/books", BookDoc)))]
pub(crate) struct ApiDoc;

pub fn route_controllers(pool: Pool<PostgresConnectionManager<NoTls>>, router: Router) -> Router {
  let doc = ApiDoc::openapi();
  router
    .nest("/books", book_controller::routes(pool))
    .merge(SwaggerUi::new("/swagger-ui")
      .url("/api-docs/openapi.json", doc))
}

fn get_book_service(pool: DatabaseConnection) -> Arc<dyn BookService> {
  Arc::new(DefaultBookService::new(get_book_repository(pool)))
}

fn get_book_repository(pool: DatabaseConnection) -> Arc<dyn BookRepository> {
  Arc::new(DefaultBookRepository::new(pool.0, DEFAULT_LANGUAGE))
}

fn format_content_language(language: Language) -> String {
  language.language_code().to_string()
}

fn convert_to_language(value: Option<&AcceptLanguage>) -> Option<Language> {
  Language::from_str(&value?.value).ok()
}

fn get_language(mut languages: Vec<AcceptLanguage>, default_language: Language) -> Language {
  languages.sort_by(|x, y| x.cmp(&y));
  let language = convert_to_language(languages.get(0)).unwrap_or(default_language);

  language
}

fn content_language_header(language: Language) -> HeaderMap {
  let mut headers = HeaderMap::new();
  insert_content_language_header(&mut headers, language);
  headers
}

fn insert_content_language_header(headers: &mut HeaderMap, language: Language) -> &HeaderMap {
  headers.insert("content-language", format_content_language(language).parse().unwrap());
  headers
}

//TODO: Make configurable
pub const DEFAULT_LANGUAGE: Language = EN;

pub fn convert_service_error(service_error: ServiceError) -> StatusCode {
  match service_error {
    ServiceError::ClientError(_) => StatusCode::BAD_REQUEST,
    ServiceError::ServerError(e) => {
      eprintln!("{}", e);
      StatusCode::INTERNAL_SERVER_ERROR
    }
  }
}

fn set_pagination_limit(pagination: &mut QueryPagination) {
  if pagination.count > 50 {
    pagination.count = 50;
  }
}

