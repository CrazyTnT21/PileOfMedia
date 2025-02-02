use crate::controllers::map_accept_languages;
use crate::controllers::map_language_header;
use std::sync::Arc;

use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{delete, get, post};
use axum::{Json, Router};
use tokio_postgres::{Client, Transaction};

use domain::entities::genre::create_genre::CreateGenre;
use domain::enums::language::Language;
use services::genre_service::mut_genre_service::MutGenreService;
use services::genre_service::GenreService;

use crate::app_state::AppState;
use crate::controllers::{
  convert_error, convert_service_error, set_pagination_limit,
};
use crate::extractors::headers::accept_language::AcceptLanguageHeader;
use crate::extractors::query_pagination::QueryPagination;
use crate::implementations::{
  get_genre_repository, get_genre_service, get_mut_genre_repository, get_mut_genre_service,
};
use crate::openapi::params::header::accept_language::AcceptLanguageParam;
use crate::openapi::params::path::id::IdParam;
use crate::openapi::params::path::name::NameParam;
use crate::openapi::params::query::count::CountParam;
use crate::openapi::params::query::page::PageParam;
use crate::openapi::responses::bad_request::BadRequest;
use crate::openapi::responses::not_found::NotFound;
use crate::openapi::responses::server_error::ServerError;

pub mod genre_doc;

pub fn routes(app_state: AppState) -> Router {
  Router::new()
    .route("/", get(get_items))
    .route("/", post(create_item))
    .route("/:id", get(get_by_id))
    .route("/:id", delete(delete_item))
    .route("/name/:name", get(get_by_name))
    .with_state(app_state)
}

#[utoipa::path(get, path = "",
  responses(
    (status = 200, description = "Returned genres", body = GenresTotal), ServerError, BadRequest),
  params(AcceptLanguageParam, PageParam, CountParam),
  tag = "Genres"
)]
async fn get_items(
  AcceptLanguageHeader(languages): AcceptLanguageHeader,
  State(app_state): State<AppState>,
  Query(mut pagination): Query<QueryPagination>,
) -> impl IntoResponse {
  let connection = app_state.pool.get().await.map_err(convert_error)?;
  let service = get_service(&connection);

  set_pagination_limit(&mut pagination);

  let languages = map_accept_languages(&languages);
  let content_language = map_language_header(&languages);
  println!("Route for genres in {}", language_text(&languages));

  match service.get(&languages, pagination.into()).await {
    Ok(genres) => Ok((StatusCode::OK, content_language, Json(genres))),
    Err(error) => Err(convert_service_error(error)),
  }
}
#[utoipa::path(get, path = "/{id}",
  responses(
    (status = 200, description = "Returned genre based on the id", body = Genre), ServerError, BadRequest, NotFound
  ),
  params(IdParam, AcceptLanguageParam),
  tag = "Genres"
)]
async fn get_by_id(
  Path(id): Path<u32>,
  AcceptLanguageHeader(languages): AcceptLanguageHeader,
  State(app_state): State<AppState>,
) -> impl IntoResponse {
  let connection = app_state.pool.get().await.map_err(convert_error)?;
  let service = get_service(&connection);

  let languages = map_accept_languages(&languages);
  let content_language = map_language_header(&languages);
  println!("Route for a genre with id {} in {}", id, language_text(&languages));

  match service.get_by_id(id, &languages).await {
    Ok(item) => match item {
      None => Err((StatusCode::NOT_FOUND, "".to_string())),
      Some(item) => Ok((StatusCode::OK, content_language, Json(item))),
    },
    Err(error) => Err(convert_service_error(error)),
  }
}

#[utoipa::path(get, path = "/name/{name}",
  responses(
    (status = 200, description = "Returned genres based on the name", body = GenresTotal), ServerError, BadRequest
  ),
  params(NameParam, AcceptLanguageParam, PageParam, CountParam),
  tag = "Genres"
)]
async fn get_by_name(
  Path(name): Path<String>,
  AcceptLanguageHeader(languages): AcceptLanguageHeader,
  State(app_state): State<AppState>,
  Query(mut pagination): Query<QueryPagination>,
) -> impl IntoResponse {
  let connection = app_state.pool.get().await.map_err(convert_error)?;
  let service = get_service(&connection);

  set_pagination_limit(&mut pagination);

  let languages = map_accept_languages(&languages);
  let content_language = map_language_header(&languages);
  println!(
    "Route for genres with the name {} in {}",
    name,
    language_text(&languages)
  );

  match service.get_by_name(&name, &languages, pagination.into()).await {
    Ok(items) => Ok((StatusCode::OK, content_language, Json(items))),
    Err(error) => Err(convert_service_error(error)),
  }
}

fn get_service(connection: &Client) -> impl GenreService + '_ {
  let repository = get_genre_repository(connection);
  get_genre_service(Arc::new(repository))
}

#[utoipa::path(post, path = "",
responses(
(status = 201, description = "Genre successfully created", body = Genre), ServerError, BadRequest
),
request_body = CreateGenre,
tag = "Genres"
)]
async fn create_item(State(app_state): State<AppState>, Json(create_genre): Json<CreateGenre>) -> impl IntoResponse {
  let mut connection = app_state.pool.get().await.map_err(convert_error)?;
  let transaction = connection.transaction().await.map_err(convert_error)?;
  let result = {
    let client = transaction.client();
    let service = get_mut_service(&transaction, client);

    println!("Route for creating a genre");

    match service.create(create_genre).await {
      Ok(genre) => Ok((StatusCode::CREATED, Json(genre))),
      Err(error) => Err(convert_service_error(error)),
    }
  };
  transaction.commit().await.map_err(convert_error)?;
  result
}

#[utoipa::path(delete, path = "/{id}",
  responses(
    (status = 204, description = "Genre successfully deleted"), ServerError, BadRequest
  ),
  params(("id" = u32, Path, description = "Id of the item to delete")),
  tag = "Genres"
)]
async fn delete_item(Path(id): Path<u32>, State(app_state): State<AppState>) -> impl IntoResponse {
  let mut connection = app_state.pool.get().await.map_err(convert_error)?;
  let transaction = connection.transaction().await.map_err(convert_error)?;
  let result = {
    let client = transaction.client();
    let service = get_mut_service(&transaction, client);

    println!("Route for deleting a genre");

    match service.delete(&[id]).await {
      Ok(()) => Ok(StatusCode::NO_CONTENT),
      Err(error) => Err(convert_service_error(error)),
    }
  };
  transaction.commit().await.map_err(convert_error)?;
  result
}

fn get_mut_service<'a>(transaction: &'a Transaction<'a>, client: &'a Client) -> impl MutGenreService + 'a {
  let genre_repository = Arc::new(get_genre_repository(client));
  let mut_genre_repository = Arc::new(get_mut_genre_repository(transaction, genre_repository.clone()));
  get_mut_genre_service(genre_repository, mut_genre_repository)
}

fn language_text(languages: &[Language]) -> String {
  languages
    .iter()
    .map(Language::language_code)
    .collect::<Vec<&str>>()
    .join(",")
}
