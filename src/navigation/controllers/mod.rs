use std::str::FromStr;
use axum::http::HeaderMap;
use axum::Router;

use crate::domain::enums::language::Language;
use crate::domain::enums::language::Language::EN;
use crate::navigation::headers::accept_language::{AcceptLanguage};

mod book_controller;

pub fn add_controllers(router: Router) -> Router {
  book_controller::add_routes(router)
}

fn format_content_language(language: Language, fallback_language: Option<Language>) -> String {
  match fallback_language {
    None => language.language_code().to_string(),
    Some(fallback) => format!("{},{}", language.language_code(), fallback.language_code()),
  }
}

fn convert_to_language(value: Option<&AcceptLanguage>) -> Option<Language> {
  Language::from_str(&value?.value).ok()
}

fn get_language_and_fallback(mut languages: Vec<AcceptLanguage>, default_language: Language) -> (Language, Option<Language>) {
  languages.sort_by(|x, y| x.cmp(&y));
  let language = convert_to_language(languages.get(0)).unwrap_or(default_language);
  let fallback_language = convert_to_language(languages.get(1));

  (language, fallback_language)
}

fn content_language_header(language: Language, fallback_language: Option<Language>) -> HeaderMap {
  let mut headers = HeaderMap::new();
  insert_content_language_header(&mut headers, language, fallback_language);
  headers
}

fn insert_content_language_header(headers: &mut HeaderMap, language: Language, fallback_language: Option<Language>) -> &HeaderMap {
  headers.insert("content-language", format_content_language(language, fallback_language).parse().unwrap());
  headers
}

//TODO: Make configurable
const DEFAULT_LANGUAGE: Language = EN;
