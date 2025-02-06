use std::sync::Arc;

use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{delete, get, post};
use axum::{Json, Router};
use tokio_postgres::{Client, Transaction};

use domain::entities::franchise::create_franchise::CreateFranchise;
use services::franchise_service::mut_franchise_service::MutFranchiseService;
use services::franchise_service::FranchiseService;

use crate::app_state::AppState;
use crate::controllers::{
  convert_error, convert_service_error, map_accept_languages, map_language_header, set_pagination_limit,
};
use crate::extractors::headers::accept_language::AcceptLanguageHeader;
use crate::extractors::query_pagination::QueryPagination;
use crate::implementations::{
  get_franchise_repository, get_franchise_service, get_mut_franchise_repository, get_mut_franchise_service,
};
use crate::openapi::params::header::accept_language::AcceptLanguageParam;
use crate::openapi::params::path::id::IdParam;
use crate::openapi::params::path::name::NameParam;
use crate::openapi::params::query::count::CountParam;
use crate::openapi::params::query::page::PageParam;
use crate::openapi::responses::bad_request::BadRequest;
use crate::openapi::responses::not_found::NotFound;
use crate::openapi::responses::server_error::ServerError;

pub mod franchise_doc;

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
    (status = 200, description = "Returned franchises", body = FranchisesTotal), ServerError, BadRequest),
  params(AcceptLanguageParam, PageParam, CountParam),
  tag = "Franchises"
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
  println!("Route for franchises in {:?}", &languages);

  match service.get(&languages, pagination.into()).await {
    Ok(franchises) => Ok((StatusCode::OK, content_language, Json(franchises))),
    Err(error) => Err(convert_service_error(error)),
  }
}

#[utoipa::path(get, path = "/{id}",
  responses(
    (status = 200, description = "Returned franchise based on the id", body = Franchise), ServerError, BadRequest, NotFound
  ),
  params(IdParam, AcceptLanguageParam),
  tag = "Franchises"
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
  println!("Route for a franchise with id {} in {:?}", id, &languages);

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
    (status = 200, description = "Returned franchises based on the name", body = FranchisesTotal), ServerError, BadRequest
  ),
  params(NameParam, AcceptLanguageParam, PageParam, CountParam),
  tag = "Franchises"
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
  println!("Route for franchises with the name {} in {:?}", name, &languages);

  match service.get_by_name(&name, &languages, pagination.into()).await {
    Ok(items) => Ok((StatusCode::OK, content_language, Json(items))),
    Err(error) => Err(convert_service_error(error)),
  }
}

fn get_service(connection: &Client) -> impl FranchiseService + '_ {
  let repository = get_franchise_repository(connection);
  get_franchise_service(Arc::new(repository))
}

#[utoipa::path(post, path = "",
responses(
(status = 201, description = "Franchise successfully created", body = Franchise), ServerError, BadRequest
),
request_body = CreateFranchise,
tag = "Franchises"
)]
async fn create_item(
  State(app_state): State<AppState>,
  Json(create_franchise): Json<CreateFranchise>,
) -> impl IntoResponse {
  let mut connection = app_state.pool.get().await.map_err(convert_error)?;
  let transaction = connection.transaction().await.map_err(convert_error)?;
  let result = {
    let client = transaction.client();
    let service = get_mut_service(&transaction, client);

    println!("Route for creating a franchise");

    match service.create(create_franchise).await {
      Ok(franchise) => Ok((StatusCode::CREATED, Json(franchise))),
      Err(error) => Err(convert_service_error(error)),
    }
  };
  transaction.commit().await.map_err(convert_error)?;
  result
}

#[utoipa::path(delete, path = "/{id}",
  responses(
    (status = 204, description = "Franchise successfully deleted"), ServerError, BadRequest
  ),
  params(("id" = u32, Path, description = "Id of the item to delete")),
  tag = "Franchises"
)]
async fn delete_item(Path(id): Path<u32>, State(app_state): State<AppState>) -> impl IntoResponse {
  let mut connection = app_state.pool.get().await.map_err(convert_error)?;
  let transaction = connection.transaction().await.map_err(convert_error)?;
  let result = {
    let client = transaction.client();
    let service = get_mut_service(&transaction, client);

    println!("Route for deleting a franchise");

    match service.delete(&[id]).await {
      Ok(()) => Ok(StatusCode::NO_CONTENT),
      Err(error) => Err(convert_service_error(error)),
    }
  };
  transaction.commit().await.map_err(convert_error)?;
  result
}

fn get_mut_service<'a>(transaction: &'a Transaction<'a>, client: &'a Client) -> impl MutFranchiseService + 'a {
  let franchise_repository = Arc::new(get_franchise_repository(client));
  let mut_franchise_repository = Arc::new(get_mut_franchise_repository(transaction, franchise_repository.clone()));
  get_mut_franchise_service(franchise_repository, mut_franchise_repository)
}
