use std::str::FromStr;
use axum::http::{HeaderMap, StatusCode};
use axum::Router;

use crate::domain::enums::language::Language;
use crate::domain::enums::language::Language::EN;
use crate::navigation::headers::accept_language::{AcceptLanguage};
use crate::services::traits::service_error::ServiceError;

mod book_controller;

pub fn add_controllers(router: Router) -> Router {
  book_controller::add_routes(router)
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
const DEFAULT_LANGUAGE: Language = EN;

pub fn convert_service_error(service_error: ServiceError) -> StatusCode {
  match service_error {
    ServiceError::ClientError(_) => StatusCode::BAD_REQUEST,
    ServiceError::ServerError(_) => StatusCode::INTERNAL_SERVER_ERROR
  }
}

