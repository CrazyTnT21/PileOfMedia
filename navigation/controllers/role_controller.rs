use std::sync::Arc;

use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{delete, get, post};
use axum::{Json, Router};
use tokio_postgres::{Client, Transaction};

use domain::entities::role::create_role::CreateRole;
use services::role_service::RoleService;
use services::role_service::mut_role_service::MutRoleService;

use crate::app_state::AppState;
use crate::controllers::{
  convert_error, convert_service_error, map_accept_languages, map_language_header, set_pagination_limit,
};
use crate::extractors::headers::accept_language::AcceptLanguageHeader;
use crate::extractors::query_pagination::QueryPagination;
use crate::implementations::{get_mut_role_repository, get_mut_role_service, get_role_repository, get_role_service};
use crate::openapi::params::header::accept_language::AcceptLanguageParam;
use crate::openapi::params::path::id::IdParam;
use crate::openapi::params::path::name::NameParam;
use crate::openapi::params::query::count::CountParam;
use crate::openapi::params::query::page::PageParam;
use crate::openapi::responses::bad_request::BadRequest;
use crate::openapi::responses::not_found::NotFound;
use crate::openapi::responses::server_error::ServerError;

pub mod role_doc;

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
    (status = 200, description = "Returned roles", body = RolesTotal), ServerError, BadRequest),
  params(AcceptLanguageParam, PageParam, CountParam),
  tag = "Roles"
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
  println!("Route for roles in {:?}", &languages);

  match service.get(&languages, pagination.into()).await {
    Ok(roles) => Ok((StatusCode::OK, content_language, Json(roles))),
    Err(error) => Err(convert_service_error(error)),
  }
}

#[utoipa::path(get, path = "/{id}",
  responses(
    (status = 200, description = "Returned role based on the id", body = Role), ServerError, BadRequest, NotFound
  ),
  params(IdParam, AcceptLanguageParam),
  tag = "Roles"
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
  println!("Route for a role with id {} in {:?}", id, &languages);

  match service.get_by_id(id, &languages).await {
    Ok(Some(item)) => Ok((StatusCode::OK, content_language, Json(item))),
    Ok(None) => Err((StatusCode::NOT_FOUND, "".to_string())),
    Err(error) => Err(convert_service_error(error)),
  }
}

#[utoipa::path(get, path = "/name/{name}",
  responses(
    (status = 200, description = "Returned roles based on the name", body = RolesTotal), ServerError, BadRequest
  ),
  params(NameParam, AcceptLanguageParam, PageParam, CountParam),
  tag = "Roles"
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
  println!("Route for roles with the name {} in {:?}", name, &languages);

  match service.get_by_name(&name, &languages, pagination.into()).await {
    Ok(items) => Ok((StatusCode::OK, content_language, Json(items))),
    Err(error) => Err(convert_service_error(error)),
  }
}

fn get_service(connection: &Client) -> impl RoleService + '_ {
  let repository = get_role_repository(connection);
  get_role_service(Arc::new(repository))
}

#[utoipa::path(post, path = "",
responses(
(status = 201, description = "Role successfully created", body = Role), ServerError, BadRequest
),
request_body = CreateRole,
tag = "Roles"
)]
async fn create_item(State(app_state): State<AppState>, Json(create_role): Json<CreateRole>) -> impl IntoResponse {
  let mut connection = app_state.pool.get().await.map_err(convert_error)?;
  let transaction = connection.transaction().await.map_err(convert_error)?;
  let result = {
    let client = transaction.client();
    let service = get_mut_service(&transaction, client);

    println!("Route for creating a role");

    match service.create(create_role).await {
      Ok(role) => Ok((StatusCode::CREATED, Json(role))),
      Err(error) => Err(convert_service_error(error)),
    }
  };
  transaction.commit().await.map_err(convert_error)?;
  result
}

#[utoipa::path(delete, path = "/{id}",
  responses(
    (status = 204, description = "Role successfully deleted"), ServerError, BadRequest
  ),
  params(("id" = u32, Path, description = "Id of the item to delete")),
  tag = "Roles"
)]
async fn delete_item(Path(id): Path<u32>, State(app_state): State<AppState>) -> impl IntoResponse {
  let mut connection = app_state.pool.get().await.map_err(convert_error)?;
  let transaction = connection.transaction().await.map_err(convert_error)?;
  let result = {
    let client = transaction.client();
    let service = get_mut_service(&transaction, client);

    println!("Route for deleting a role");

    match service.delete(&[id]).await {
      Ok(()) => Ok(StatusCode::NO_CONTENT),
      Err(error) => Err(convert_service_error(error)),
    }
  };
  transaction.commit().await.map_err(convert_error)?;
  result
}

fn get_mut_service<'a>(transaction: &'a Transaction<'a>, client: &'a Client) -> impl MutRoleService + 'a {
  let role_repository = Arc::new(get_role_repository(client));
  let mut_role_repository = Arc::new(get_mut_role_repository(transaction, role_repository.clone()));
  get_mut_role_service(role_repository, mut_role_repository)
}
