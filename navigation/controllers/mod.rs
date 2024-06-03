use std::str::FromStr;

use axum::http::{HeaderMap, StatusCode};
use axum::Router;
use bb8_postgres::bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::NoTls;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use domain::enums::language::Language;
use domain::enums::language::Language::EN;
use services::traits::service_error::ServiceError;

use crate::extractors::headers::accept_language::AcceptLanguage;
use crate::extractors::query_pagination::QueryPagination;

mod doc;
mod book_controller;
mod genre_controller;
mod theme_controller;
mod person_controller;

pub fn route_controllers(pool: Pool<PostgresConnectionManager<NoTls>>, router: Router) -> Router {
  let doc = doc::ApiDoc::openapi();
  router
    .nest("/books", book_controller::routes(pool.clone()))
    .nest("/genres", genre_controller::routes(pool.clone()))
    .nest("/themes", theme_controller::routes(pool.clone()))
    .nest("/people", person_controller::routes(pool))
    .merge(SwaggerUi::new("/swagger-ui")
      .url("/api-docs/openapi.json", doc))
}

fn convert_to_language(value: Option<&AcceptLanguage>) -> Option<Language> {
  Language::from_str(&value?.value).ok()
}

fn get_language(mut languages: Vec<AcceptLanguage>, default_language: Language) -> Language {
  languages.sort();
  let language = convert_to_language(languages.first()).unwrap_or(default_language);

  language
}

fn content_language_header(language: Language) -> HeaderMap {
  let mut headers = HeaderMap::new();
  append_content_language_header(&mut headers, language);
  headers
}

fn append_content_language_header(headers: &mut HeaderMap, language: Language) -> &HeaderMap {
  let mut value = language.language_code().to_string();
  if let Some(header_value) = headers.get("content-language") {
    value.push(',');
    value.push_str(header_value.to_str().unwrap());
  }

  headers.insert("content-language", value.parse().unwrap());
  headers
}

//TODO: Make configurable
pub const DEFAULT_LANGUAGE: Language = EN;

pub fn convert_service_error(service_error: ServiceError) -> StatusCode {
  match service_error {
    ServiceError::ClientError(_) => StatusCode::BAD_REQUEST,
    ServiceError::ServerError(e) => {
      eprintln!("Error: {e}");
      StatusCode::INTERNAL_SERVER_ERROR
    }
  }
}

fn set_pagination_limit(pagination: &mut QueryPagination) {
  if pagination.count > 50 {
    pagination.count = 50;
  }
}

