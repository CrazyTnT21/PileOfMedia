use std::sync::Arc;

use axum::{Json, Router};
use axum::extract::{Path, Query};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use bb8_postgres::bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::{Client, NoTls};

use services::user_service::UserService;

use crate::controllers::{convert_service_error, set_pagination_limit};
use crate::database_connection::DatabaseConnection;
use crate::extractors::query_pagination::QueryPagination;
use crate::implementations::{get_image_repository, get_user_repository, get_user_service};
use crate::openapi::params::path::id::IdParam;
use crate::openapi::params::path::name::NameParam;
use crate::openapi::params::query::count::CountParam;
use crate::openapi::params::query::page::PageParam;
use crate::openapi::responses::bad_request::BadRequest;
use crate::openapi::responses::not_found::NotFound;
use crate::openapi::responses::server_error::ServerError;

pub mod user_doc;

pub fn routes(pool: Pool<PostgresConnectionManager<NoTls>>) -> Router {
  Router::new()
    .route("/", get(get_items))
    .route("/:id", get(get_by_id))
    .route("/name/:name", get(get_by_name))
    .with_state(pool)
}

#[utoipa::path(get, path = "",
  responses(
    (status = 200, description = "Returned users", body = UsersTotal), ServerError, BadRequest),
  params(PageParam, CountParam),
  tag = "Users"
)]
async fn get_items(connection: DatabaseConnection, Query(mut pagination): Query<QueryPagination>) -> impl IntoResponse {
  let connection = connection.0;
  let service = get_service(&connection);

  set_pagination_limit(&mut pagination);

  println!("Route for users");

  match service.get(pagination.into()).await {
    Ok(users) => Ok((StatusCode::OK, Json(users))),
    Err(error) => Err(convert_service_error(error))
  }
}

#[utoipa::path(get, path = "/{id}",
  responses(
    (status = 200, description = "Returned user based on the id", body = User), ServerError, BadRequest, NotFound
  ),
  params(IdParam),
  tag = "Users"
)]
async fn get_by_id(Path(id): Path<u32>, connection: DatabaseConnection) -> impl IntoResponse {
  let connection = connection.0;
  let service = get_service(&connection);

  println!("Route for a user with id {}", id, );

  match service.get_by_id(id).await {
    Ok(item) => Ok((StatusCode::OK, Json(item))),
    Err(error) => Err(convert_service_error(error))
  }
}

#[utoipa::path(get, path = "/name/{name}",
  responses(
    (status = 200, description = "Returned users based on the name", body = UsersTotal), ServerError, BadRequest
  ),
  params(NameParam, PageParam, CountParam),
  tag = "Users"
)]
async fn get_by_name(Path(name): Path<String>, connection: DatabaseConnection, Query(mut pagination): Query<QueryPagination>) -> impl IntoResponse {
  let connection = connection.0;
  let service = get_service(&connection);

  set_pagination_limit(&mut pagination);

  println!("Route for users with the name {}", name);

  match service.get_by_name(&name, pagination.into()).await {
    Ok(items) => Ok((StatusCode::OK, Json(items))),
    Err(error) => Err(convert_service_error(error))
  }
}

fn get_service(connection: &Client) -> impl UserService + '_ {
  let image_repository = Arc::new(get_image_repository(connection));
  let repository = get_user_repository(connection, image_repository);
  get_user_service(Arc::new(repository))
}
