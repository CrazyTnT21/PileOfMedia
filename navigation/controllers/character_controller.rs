use domain::entities::character::create_character::CreateCharacter;
use multipart::MultiPartRequest;
use std::sync::Arc;

use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{delete, get, post};
use axum::{Json, Router};
use tokio_postgres::{Client, Transaction};

use crate::app_state::AppState;
use crate::controllers::{
  convert_error, convert_service_error, map_accept_languages, map_language_header, set_pagination_limit,
};
use crate::extractors::headers::accept_language::AcceptLanguageHeader;
use crate::extractors::query_pagination::QueryPagination;
use crate::implementations::{
  get_character_repository, get_character_service, get_file_repository, get_image_repository,
  get_mut_character_repository, get_mut_character_service, get_mut_file_repository, get_mut_file_service,
  get_mut_image_repository, get_mut_image_service,
};
use crate::openapi::params::header::accept_language::AcceptLanguageParam;
use crate::openapi::params::path::id::IdParam;
use crate::openapi::params::path::name::NameParam;
use crate::openapi::params::query::count::CountParam;
use crate::openapi::params::query::page::PageParam;
use crate::openapi::responses::bad_request::BadRequest;
use crate::openapi::responses::not_found::NotFound;
use crate::openapi::responses::server_error::ServerError;
use services::character_service::CharacterService;
use services::character_service::mut_character_service::MutCharacterService;

pub mod character_doc;

pub fn routes(app_state: AppState) -> Router {
  Router::new()
    .route("/", get(get_items))
    .route("/", post(create_item))
    .route("/{id}", get(get_by_id))
    .route("/{id}", delete(delete_item))
    .route("/name/{name}", get(get_by_name))
    .with_state(app_state)
}

#[utoipa::path(get, path = "",
  responses(
    (status = 200, description = "Returned characters", body = CharactersTotal), ServerError, BadRequest),
  params(AcceptLanguageParam, PageParam, CountParam),
  tag = "Characters"
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
  println!("Route for characters in {:?}", &languages);

  match service.get(&languages, pagination.into()).await {
    Ok(characters) => Ok((StatusCode::OK, content_language, Json(characters))),
    Err(error) => Err(convert_service_error(error)),
  }
}

#[utoipa::path(get, path = "/{id}",
  responses(
    (status = 200, description = "Returned character based on the id", body = Character), ServerError, BadRequest, NotFound
  ),
  params(IdParam, AcceptLanguageParam),
  tag = "Characters"
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
  println!("Route for a character with id {} in {:?}", id, &languages);

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
    (status = 200, description = "Returned characters based on the name", body = CharactersTotal), ServerError, BadRequest
  ),
  params(NameParam, AcceptLanguageParam, PageParam, CountParam),
  tag = "Characters"
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
  println!("Route for characters with the name {} in {:?}", name, &languages);

  match service.get_by_name(&name, &languages, pagination.into()).await {
    Ok(items) => Ok((StatusCode::OK, content_language, Json(items))),
    Err(error) => Err(convert_service_error(error)),
  }
}

#[utoipa::path(post, path = "",
  responses(
    (status = 201, description = "Character successfully created", body = Character), ServerError, BadRequest
  ),
  request_body(content_type = ["multipart/form-data"], content = CreateCharacter),
  tag = "Characters"
)]
async fn create_item(
  State(app_state): State<AppState>,
  MultiPartRequest(create_character): MultiPartRequest<CreateCharacter>,
) -> impl IntoResponse {
  let mut connection = app_state.pool.get().await.map_err(convert_error)?;
  let transaction = connection.transaction().await.map_err(convert_error)?;
  let result = {
    let client = transaction.client();
    let service = get_mut_service(&transaction, client, &app_state.display_path, &app_state.content_path);

    println!("Route for creating a character");

    match service.create(create_character).await {
      Ok(character) => Ok((StatusCode::CREATED, Json(character))),
      Err(error) => Err(convert_service_error(error)),
    }
  };
  transaction.commit().await.map_err(convert_error)?;
  result
}

#[utoipa::path(delete, path = "/{id}",
  responses(
    (status = 204, description = "Character successfully deleted"), ServerError, BadRequest
  ),
  params(("id" = u32, Path, description = "Id of the item to delete")),
  tag = "Characters"
)]
async fn delete_item(Path(id): Path<u32>, State(app_state): State<AppState>) -> impl IntoResponse {
  let mut connection = app_state.pool.get().await.map_err(convert_error)?;
  let transaction = connection.transaction().await.map_err(convert_error)?;
  let result = {
    let client = transaction.client();
    let service = get_mut_service(&transaction, client, &app_state.display_path, &app_state.content_path);

    println!("Route for deleting a character");

    match service.delete(&[id]).await {
      Ok(()) => Ok(StatusCode::NO_CONTENT),
      Err(error) => Err(convert_service_error(error)),
    }
  };
  transaction.commit().await.map_err(convert_error)?;
  result
}

fn get_service(connection: &Client) -> impl CharacterService + '_ {
  let image_repository = Arc::new(get_image_repository(connection));
  let repository = get_character_repository(connection, image_repository);
  get_character_service(Arc::new(repository))
}

fn get_mut_service<'a>(
  transaction: &'a Transaction<'a>,
  client: &'a Client,
  display_path: &'a str,
  path: &'a str,
) -> impl MutCharacterService + 'a {
  let image_repository = Arc::new(get_image_repository(client));
  let character_repository = Arc::new(get_character_repository(client, image_repository.clone()));
  let mut_character_repository = Arc::new(get_mut_character_repository(transaction, character_repository.clone()));
  let mut_file_repository = Arc::new(get_mut_file_repository());
  let file_repository = Arc::new(get_file_repository());
  let mut_image_repository = Arc::new(get_mut_image_repository(
    transaction,
    image_repository,
    mut_file_repository.clone(),
    file_repository,
  ));
  let mut_file_service = Arc::new(get_mut_file_service(mut_file_repository));
  let mut_image_service = Arc::new(get_mut_image_service(
    mut_image_repository,
    mut_file_service,
    display_path,
    path,
  ));
  get_mut_character_service(character_repository, mut_character_repository, mut_image_service)
}
