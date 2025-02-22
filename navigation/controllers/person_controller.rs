use domain::entities::person::create_person::CreatePerson;
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
  get_file_repository, get_image_repository, get_mut_file_repository, get_mut_file_service, get_mut_image_repository,
  get_mut_image_service, get_mut_person_repository, get_mut_person_service, get_person_repository, get_person_service,
};
use crate::openapi::params::header::accept_language::AcceptLanguageParam;
use crate::openapi::params::path::id::IdParam;
use crate::openapi::params::path::name::NameParam;
use crate::openapi::params::query::count::CountParam;
use crate::openapi::params::query::page::PageParam;
use crate::openapi::responses::bad_request::BadRequest;
use crate::openapi::responses::not_found::NotFound;
use crate::openapi::responses::server_error::ServerError;
use services::person_service::PersonService;
use services::person_service::mut_person_service::MutPersonService;

pub mod person_doc;

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
    (status = 200, description = "Returned people", body = PeopleTotal), ServerError, BadRequest),
  params(AcceptLanguageParam, PageParam, CountParam),
  tag = "People"
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
  println!("Route for people in {:?}", &languages);

  match service.get(&languages, pagination.into()).await {
    Ok(people) => Ok((StatusCode::OK, content_language, Json(people))),
    Err(error) => Err(convert_service_error(error)),
  }
}

#[utoipa::path(get, path = "/{id}",
  responses(
    (status = 200, description = "Returned person based on the id", body = Person), ServerError, BadRequest, NotFound
  ),
  params(IdParam, AcceptLanguageParam),
  tag = "People"
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
  println!("Route for a person with id {} in {:?}", id, &languages);

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
    (status = 200, description = "Returned people based on the name", body = PeopleTotal), ServerError, BadRequest
  ),
  params(NameParam, AcceptLanguageParam, PageParam, CountParam),
  tag = "People"
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
  println!("Route for people with the name {} in {:?}", name, &languages);

  match service.get_by_name(&name, &languages, pagination.into()).await {
    Ok(items) => Ok((StatusCode::OK, content_language, Json(items))),
    Err(error) => Err(convert_service_error(error)),
  }
}

#[utoipa::path(post, path = "",
  responses(
    (status = 201, description = "Person successfully created", body = Person), ServerError, BadRequest
  ),
  request_body(content_type = ["multipart/form-data"], content = CreatePerson),
  tag = "People"
)]
async fn create_item(
  State(app_state): State<AppState>,
  MultiPartRequest(create_person): MultiPartRequest<CreatePerson>,
) -> impl IntoResponse {
  let mut connection = app_state.pool.get().await.map_err(convert_error)?;
  let transaction = connection.transaction().await.map_err(convert_error)?;
  let result = {
    let client = transaction.client();
    let service = get_mut_service(&transaction, client, &app_state.display_path, &app_state.content_path);

    println!("Route for creating a person");

    match service.create(create_person).await {
      Ok(person) => Ok((StatusCode::CREATED, Json(person))),
      Err(error) => Err(convert_service_error(error)),
    }
  };
  transaction.commit().await.map_err(convert_error)?;
  result
}

#[utoipa::path(delete, path = "/{id}",
  responses(
    (status = 204, description = "Person successfully deleted"), ServerError, BadRequest
  ),
  params(("id" = u32, Path, description = "Id of the item to delete")),
  tag = "People"
)]
async fn delete_item(Path(id): Path<u32>, State(app_state): State<AppState>) -> impl IntoResponse {
  let mut connection = app_state.pool.get().await.map_err(convert_error)?;
  let transaction = connection.transaction().await.map_err(convert_error)?;
  let result = {
    let client = transaction.client();
    let service = get_mut_service(&transaction, client, &app_state.display_path, &app_state.content_path);

    println!("Route for deleting a person");

    match service.delete(&[id]).await {
      Ok(()) => Ok(StatusCode::NO_CONTENT),
      Err(error) => Err(convert_service_error(error)),
    }
  };
  transaction.commit().await.map_err(convert_error)?;
  result
}

fn get_service(connection: &Client) -> impl PersonService + '_ {
  let image_repository = Arc::new(get_image_repository(connection));
  let repository = get_person_repository(connection, image_repository);
  get_person_service(Arc::new(repository))
}

fn get_mut_service<'a>(
  transaction: &'a Transaction<'a>,
  client: &'a Client,
  display_path: &'a str,
  path: &'a str,
) -> impl MutPersonService + 'a {
  let image_repository = Arc::new(get_image_repository(client));
  let person_repository = Arc::new(get_person_repository(client, image_repository.clone()));
  let mut_person_repository = Arc::new(get_mut_person_repository(transaction, person_repository.clone()));
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
  get_mut_person_service(person_repository, mut_person_repository, mut_image_service)
}
