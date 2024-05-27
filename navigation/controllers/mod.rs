use std::str::FromStr;

use axum::http::{HeaderMap, HeaderValue, StatusCode};
use axum::Router;
use bb8_postgres::bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::NoTls;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use application::Pooled;
use application::repositories::default_book_repository::DefaultBookRepository;
use application::repositories::default_image_repository::DefaultImageRepository;
use domain::entities::character::Character;
use domain::entities::genre::Genre;
use domain::entities::person::Person;
use domain::entities::theme::Theme;
use domain::enums::language::Language;
use domain::enums::language::Language::EN;
use infrastructure::services::default_book_service::DefaultBookService;
use repositories::book_repository::BookRepository;
use repositories::image_repository::ImageRepository;
use services::book_service::BookService;
use services::traits::service_error::ServiceError;

use crate::controllers::book_controller::BookDoc;
use crate::extractors::headers::accept_language::AcceptLanguage;
use crate::extractors::query_pagination::QueryPagination;

mod book_controller;

#[derive(utoipa::OpenApi)]
#[openapi(info(title = "mycollection"),
nest(("/books", BookDoc)),
components(schemas(Genre, Character, Person, Theme)))]
pub(crate) struct ApiDoc;

pub fn route_controllers(pool: Pool<PostgresConnectionManager<NoTls>>, router: Router) -> Router {
  let doc = ApiDoc::openapi();
  router
    .nest("/books", book_controller::routes(pool))
    .merge(SwaggerUi::new("/swagger-ui")
      .url("/api-docs/openapi.json", doc))
}

fn get_book_service(book_repository: &impl BookRepository) -> impl BookService + '_ {
  DefaultBookService::new(book_repository)
}

fn get_book_repository<'a>(pool: &'a Pooled, image_repository: &'a impl ImageRepository) -> impl BookRepository + 'a {
  DefaultBookRepository::new(pool, DEFAULT_LANGUAGE, image_repository)
}

fn get_image_repository<'a>(pool: &'a Pooled) -> impl ImageRepository + 'a {
  DefaultImageRepository::new(pool)
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
    value.push_str(",");
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

