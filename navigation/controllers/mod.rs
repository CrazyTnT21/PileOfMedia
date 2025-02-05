use std::error::Error;
use std::fmt::Display;
use std::str::FromStr;

use axum::http::{HeaderMap, StatusCode};
use axum::Router;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use domain::enums::language::Language;
use services::traits::service_error::ServiceError;

use crate::app_state::AppState;
use crate::extractors::headers::accept_language::AcceptLanguage;
use crate::extractors::query_pagination::QueryPagination;

mod account_controller;
mod book_controller;
mod character_controller;
mod doc;
mod franchise_controller;
mod genre_controller;
mod person_controller;
mod role_controller;
mod theme_controller;
mod user_controller;
pub fn generate_openapi_spec() -> Result<String, impl Error> {
  doc::ApiDoc::openapi().to_pretty_json()
}
pub fn route_controllers(app_state: AppState) -> Router {
  let doc = doc::ApiDoc::openapi();
  Router::new()
    .nest("/books", book_controller::routes(app_state.clone()))
    .nest("/genres", genre_controller::routes(app_state.clone()))
    .nest("/themes", theme_controller::routes(app_state.clone()))
    .nest("/people", person_controller::routes(app_state.clone()))
    .nest("/characters", character_controller::routes(app_state.clone()))
    .nest("/roles", role_controller::routes(app_state.clone()))
    .nest("/users", user_controller::routes(app_state.clone()))
    .nest("/franchises", franchise_controller::routes(app_state.clone()))
    .nest("/accounts", account_controller::routes(app_state))
    .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", doc))
}

fn convert_to_language(value: &AcceptLanguage) -> Option<Language> {
  Language::from_str(&value.value).ok()
}

fn map_accept_languages(languages: &[AcceptLanguage]) -> Vec<Language> {
  languages.iter().filter_map(convert_to_language).collect()
}
fn map_language_header(languages: &[Language]) -> HeaderMap {
  let mut headers = HeaderMap::new();
  for lang in languages {
    append_content_language_header(&mut headers, *lang);
  }
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

pub fn convert_service_error<T: Display>(service_error: ServiceError<T>) -> (StatusCode, String) {
  match service_error {
    ServiceError::ClientError(error) => (StatusCode::BAD_REQUEST, error.to_string()),
    ServiceError::ServerError(e) => {
      eprintln!("Error: {e}");
      (StatusCode::INTERNAL_SERVER_ERROR, "".to_string())
    }
  }
}

pub fn convert_error(error: impl Error) -> (StatusCode, String) {
  eprintln!("Error: {error}");
  (StatusCode::INTERNAL_SERVER_ERROR, "".to_string())
}

fn set_pagination_limit(pagination: &mut QueryPagination) {
  if pagination.count > 50 {
    pagination.count = 50;
  }
}
