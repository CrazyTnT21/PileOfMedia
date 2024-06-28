use std::sync::Arc;

use axum::{Json, Router};
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use tokio_postgres::Client;

use services::theme_service::ThemeService;

use crate::app_state::AppState;
use crate::controllers::{append_content_language_header, content_language_header, convert_error, convert_service_error, DEFAULT_LANGUAGE, get_language, set_pagination_limit};
use crate::extractors::headers::accept_language::AcceptLanguageHeader;
use crate::extractors::query_pagination::QueryPagination;
use crate::implementations::{get_theme_repository, get_theme_service};
use crate::openapi::params::header::accept_language::AcceptLanguageParam;
use crate::openapi::params::path::id::IdParam;
use crate::openapi::params::path::name::NameParam;
use crate::openapi::params::query::count::CountParam;
use crate::openapi::params::query::page::PageParam;
use crate::openapi::responses::bad_request::BadRequest;
use crate::openapi::responses::not_found::NotFound;
use crate::openapi::responses::server_error::ServerError;

pub mod theme_doc;

pub fn routes(app_state: AppState) -> Router {
  Router::new()
    .route("/", get(get_items))
    .route("/:id", get(get_by_id))
    .route("/name/:name", get(get_by_name))
    .with_state(app_state)
}

#[utoipa::path(get, path = "",
  responses(
    (status = 200, description = "Returned themes", body = ThemesTotal), ServerError, BadRequest),
  params(AcceptLanguageParam, PageParam, CountParam),
  tag = "Themes"
)]
async fn get_items(AcceptLanguageHeader(languages): AcceptLanguageHeader, State(app_state): State<AppState>, Query(mut pagination): Query<QueryPagination>) -> impl IntoResponse {
  let connection = app_state.pool.get().await.map_err(convert_error)?;
  let service = get_service(&connection);

  let language = get_language(languages, DEFAULT_LANGUAGE);
  set_pagination_limit(&mut pagination);

  println!("Route for themes in {}", language);

  let mut content_language = content_language_header(language);
  append_content_language_header(&mut content_language, DEFAULT_LANGUAGE);

  match service.get(language, pagination.into()).await {
    Ok(themes) => Ok((StatusCode::OK, content_language, Json(themes))),
    Err(error) => Err(convert_service_error(error))
  }
}

#[utoipa::path(get, path = "/{id}",
  responses(
    (status = 200, description = "Returned theme based on the id", body = Theme), ServerError, BadRequest, NotFound
  ),
  params(IdParam, AcceptLanguageParam),
  tag = "Themes"
)]
async fn get_by_id(Path(id): Path<u32>, AcceptLanguageHeader(languages): AcceptLanguageHeader, State(app_state): State<AppState>) -> impl IntoResponse {
  let connection = app_state.pool.get().await.map_err(convert_error)?;
  let service = get_service(&connection);

  let language = get_language(languages, DEFAULT_LANGUAGE);

  println!("Route for a theme with id {} in {}", id, language);

  let mut content_language = content_language_header(language);
  append_content_language_header(&mut content_language, DEFAULT_LANGUAGE);

  match service.get_by_id(id, language).await {
    Ok(item) => match item {
      None => Err((StatusCode::NOT_FOUND, "".to_string())),
      Some(item) => Ok((StatusCode::OK, content_language, Json(item)))
    },
    Err(error) => Err(convert_service_error(error))
  }
}


#[utoipa::path(get, path = "/name/{name}",
  responses(
    (status = 200, description = "Returned themes based on the name", body = ThemesTotal), ServerError, BadRequest
  ),
  params(NameParam, AcceptLanguageParam, PageParam, CountParam),
  tag = "Themes"
)]
async fn get_by_name(Path(name): Path<String>, AcceptLanguageHeader(languages): AcceptLanguageHeader, State(app_state): State<AppState>, Query(mut pagination): Query<QueryPagination>) -> impl IntoResponse {
  let connection = app_state.pool.get().await.map_err(convert_error)?;
  let service = get_service(&connection);

  let language = get_language(languages, DEFAULT_LANGUAGE);
  set_pagination_limit(&mut pagination);

  println!("Route for themes with the name {} in {}", name, language);

  let mut content_language = content_language_header(language);
  append_content_language_header(&mut content_language, DEFAULT_LANGUAGE);

  match service.get_by_name(&name, language, pagination.into()).await {
    Ok(items) => Ok((StatusCode::OK, content_language, Json(items))),
    Err(error) => Err(convert_service_error(error))
  }
}

fn get_service(connection: &Client) -> impl ThemeService + '_ {
  let repository = get_theme_repository(connection, DEFAULT_LANGUAGE);
  get_theme_service(Arc::new(repository))
}

