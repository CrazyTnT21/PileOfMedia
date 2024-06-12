use std::error::Error;
use std::str::FromStr;

use axum::http::{HeaderMap, StatusCode};
use axum::Router;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use domain::enums::language::Language;
use domain::enums::language::Language::EN;
use services::traits::service_error::ServiceError;

use crate::app_state::AppState;
use crate::extractors::headers::accept_language::AcceptLanguage;
use crate::extractors::query_pagination::QueryPagination;

mod doc;
mod book_controller;
mod genre_controller;
mod theme_controller;
mod person_controller;
mod character_controller;
mod role_controller;
mod user_controller;
mod account_controller;

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
    .nest("/accounts", account_controller::routes(app_state))
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

pub fn convert_service_error(service_error: ServiceError) -> (StatusCode, String) {
  match service_error {
    ServiceError::ClientError(error) => (StatusCode::BAD_REQUEST, error.title),
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

