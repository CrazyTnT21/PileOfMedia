use std::sync::Arc;

use axum::{Json, Router};
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{delete, get, post};
use tokio_postgres::{Client, Transaction};

use domain::entities::role::create_role::CreateRole;
use services::role_service::mut_role_service::MutRoleService;
use services::role_service::RoleService;

use crate::app_state::AppState;
use crate::controllers::{append_content_language_header, content_language_header, convert_error, convert_service_error, DEFAULT_LANGUAGE, get_language, set_pagination_limit};
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
    .route("/:id", get(get_by_id))
    .route("/:id", delete(delete_item))
    .route("/name/:name", get(get_by_name))
    .with_state(app_state)
}

#[utoipa::path(get, path = "",
  responses(
    (status = 200, description = "Returned roles", body = RolesTotal), ServerError, BadRequest),
  params(AcceptLanguageParam, PageParam, CountParam),
  tag = "Roles"
)]
async fn get_items(AcceptLanguageHeader(languages): AcceptLanguageHeader, State(app_state): State<AppState>, Query(mut pagination): Query<QueryPagination>) -> impl IntoResponse {
  let connection = app_state.pool.get().await.map_err(convert_error)?;
  let service = get_service(&connection);

  let language = get_language(languages, DEFAULT_LANGUAGE);
  set_pagination_limit(&mut pagination);

  println!("Route for roles in {}", language);

  let mut content_language = content_language_header(language);
  append_content_language_header(&mut content_language, DEFAULT_LANGUAGE);

  match service.get(language, pagination.into()).await {
    Ok(roles) => Ok((StatusCode::OK, content_language, Json(roles))),
    Err(error) => Err(convert_service_error(error))
  }
}

#[utoipa::path(get, path = "/{id}",
  responses(
    (status = 200, description = "Returned role based on the id", body = Role), ServerError, BadRequest, NotFound
  ),
  params(IdParam, AcceptLanguageParam),
  tag = "Roles"
)]
async fn get_by_id(Path(id): Path<u32>, AcceptLanguageHeader(languages): AcceptLanguageHeader, State(app_state): State<AppState>) -> impl IntoResponse {
  let connection = app_state.pool.get().await.map_err(convert_error)?;
  let service = get_service(&connection);

  let language = get_language(languages, DEFAULT_LANGUAGE);

  println!("Route for a role with id {} in {}", id, language);

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
    (status = 200, description = "Returned roles based on the name", body = RolesTotal), ServerError, BadRequest
  ),
  params(NameParam, AcceptLanguageParam, PageParam, CountParam),
  tag = "Roles"
)]
async fn get_by_name(Path(name): Path<String>, AcceptLanguageHeader(languages): AcceptLanguageHeader, State(app_state): State<AppState>, Query(mut pagination): Query<QueryPagination>) -> impl IntoResponse {
  let connection = app_state.pool.get().await.map_err(convert_error)?;
  let service = get_service(&connection);

  let language = get_language(languages, DEFAULT_LANGUAGE);
  set_pagination_limit(&mut pagination);

  println!("Route for roles with the name {} in {}", name, language);

  let mut content_language = content_language_header(language);
  append_content_language_header(&mut content_language, DEFAULT_LANGUAGE);

  match service.get_by_name(&name, language, pagination.into()).await {
    Ok(items) => Ok((StatusCode::OK, content_language, Json(items))),
    Err(error) => Err(convert_service_error(error))
  }
}

fn get_service(connection: &Client) -> impl RoleService + '_ {
  let repository = get_role_repository(connection, DEFAULT_LANGUAGE);
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
      Err(error) => Err(convert_service_error(error))
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
      Ok(_) => Ok(StatusCode::NO_CONTENT),
      Err(error) => Err(convert_service_error(error))
    }
  };
  transaction.commit().await.map_err(convert_error)?;
  result
}

fn get_mut_service<'a>(transaction: &'a Transaction<'a>, client: &'a Client) -> impl MutRoleService + 'a {
  let role_repository = Arc::new(get_role_repository(client, DEFAULT_LANGUAGE));
  let mut_role_repository = Arc::new(get_mut_role_repository(transaction, DEFAULT_LANGUAGE, role_repository.clone()));
  get_mut_role_service(DEFAULT_LANGUAGE, role_repository, mut_role_repository)
}
