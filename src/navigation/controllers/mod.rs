use actix_web::http::header::{AcceptLanguage, LanguageTag, Preference, QualityItem};
use actix_web::web;

use crate::domain::enums::language::Language;
use crate::domain::enums::language::Language::EN;

mod book_controller;

pub fn add_controllers(config: &mut web::ServiceConfig) {
  config.configure(book_controller::add_routes);
}

fn format_content_language(language: Language, fallback_language: Option<Language>) -> String {
  match fallback_language {
    None => language.language_code().to_string(),
    Some(fallback) => format!("{},{}", language.language_code(), fallback.language_code()),
  }
}

fn covert_to_language(value: Option<&QualityItem<Preference<LanguageTag>>>) -> Option<Language> {
  Language::from_language_tag(value?.item.item()?).ok()
}

fn get_language_and_fallback(accept_language: &mut web::Header<AcceptLanguage>, default_language: Language) -> (Language, Option<Language>) {
  accept_language.sort_by(|x, y| x.quality.cmp(&y.quality).reverse());
  let language = covert_to_language(accept_language.get(0)).unwrap_or(default_language);
  let fallback_language = covert_to_language(accept_language.get(1));

  (language, fallback_language)
}

//TODO: Make configurable
const DEFAULT_LANGUAGE: Language = EN;
