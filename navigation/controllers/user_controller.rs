use std::sync::Arc;

use crate::app_state::AppState;
use crate::controllers::{
  convert_error, convert_service_error, map_accept_languages, map_language_header, set_pagination_limit,
};
use crate::extractors::headers::accept_language::AcceptLanguageHeader;
use crate::extractors::query_pagination::QueryPagination;
use crate::implementations::{
  get_book_character_repository, get_book_genre_repository, get_book_involved_repository, get_book_repository,
  get_book_theme_repository, get_character_repository, get_franchise_repository, get_genre_repository,
  get_image_repository, get_mut_user_book_repository, get_mut_user_book_service, get_person_repository,
  get_role_repository, get_theme_repository, get_user_book_repository, get_user_book_service, get_user_repository,
  get_user_service,
};
use crate::openapi::params::header::accept_language::AcceptLanguageParam;
use crate::openapi::params::path::id::IdParam;
use crate::openapi::params::path::name::NameParam;
use crate::openapi::params::query::count::CountParam;
use crate::openapi::params::query::page::PageParam;
use crate::openapi::responses::bad_request::BadRequest;
use crate::openapi::responses::not_found::NotFound;
use crate::openapi::responses::server_error::ServerError;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{delete, get, post};
use axum::{Json, Router};
use domain::entities::user::create_user_book::CreateUserBook;
use services::user_service::user_book_service::mut_user_book_service::MutUserBookService;
use services::user_service::user_book_service::UserBookService;
use services::user_service::UserService;
use tokio_postgres::{Client, Transaction};

pub mod user_doc;

pub fn routes(app_state: AppState) -> Router {
  Router::new()
    .route("/", get(get_items))
    .route("/{id}", get(get_by_id))
    .route("/{id}/books", get(get_books))
    .route("/{id}/books", post(add_book))
    .route("/{id}/books/{book_id}", get(get_book_by_id))
    .route("/{id}/books/{book_id}", delete(remove_book))
    .route("/name/{name}", get(get_by_name))
    .with_state(app_state)
}

#[utoipa::path(get, path = "",
  responses(
    (status = 200, description = "Returned users", body = UsersTotal), ServerError, BadRequest),
  params(PageParam, CountParam),
  tag = "Users"
)]
async fn get_items(
  State(app_state): State<AppState>,
  Query(mut pagination): Query<QueryPagination>,
) -> impl IntoResponse {
  let connection = app_state.pool.get().await.map_err(convert_error)?;
  let service = get_service(&connection);

  set_pagination_limit(&mut pagination);

  println!("Route for users");

  match service.get(pagination.into()).await {
    Ok(users) => Ok((StatusCode::OK, Json(users))),
    Err(error) => Err(convert_service_error(error)),
  }
}

#[utoipa::path(get, path = "/{id}",
  responses(
    (status = 200, description = "Returned user based on the id", body = User), ServerError, BadRequest, NotFound
  ),
  params(IdParam),
  tag = "Users"
)]
async fn get_by_id(Path(id): Path<u32>, State(app_state): State<AppState>) -> impl IntoResponse {
  let connection = app_state.pool.get().await.map_err(convert_error)?;
  let service = get_service(&connection);

  println!("Route for a user with id {}", id,);

  match service.get_by_id(id).await {
    Ok(item) => match item {
      None => Err((StatusCode::NOT_FOUND, "".to_string())),
      Some(item) => Ok((StatusCode::OK, Json(item))),
    },
    Err(error) => Err(convert_service_error(error)),
  }
}

#[utoipa::path(get, path = "/name/{name}",
  responses(
    (status = 200, description = "Returned users based on the name", body = UsersTotal), ServerError, BadRequest
  ),
  params(NameParam, PageParam, CountParam),
  tag = "Users"
)]
async fn get_by_name(
  Path(name): Path<String>,
  State(app_state): State<AppState>,
  Query(mut pagination): Query<QueryPagination>,
) -> impl IntoResponse {
  let connection = app_state.pool.get().await.map_err(convert_error)?;
  let service = get_service(&connection);

  set_pagination_limit(&mut pagination);

  println!("Route for users with the name {}", name);

  match service.get_by_name(&name, pagination.into()).await {
    Ok(items) => Ok((StatusCode::OK, Json(items))),
    Err(error) => Err(convert_service_error(error)),
  }
}
#[utoipa::path(get, path = "/{id}/books",
  responses(
    (status = 200, description = "User books based on the user id", body = Vec < UserBook >), ServerError, BadRequest
  ),
  params(IdParam, AcceptLanguageParam),
  tag = "Users"
)]
async fn get_books(
  Path(id): Path<u32>,
  AcceptLanguageHeader(languages): AcceptLanguageHeader,
  State(app_state): State<AppState>,
) -> impl IntoResponse {
  let connection = app_state.pool.get().await.map_err(convert_error)?;
  let service = get_book_service(&connection);

  let languages = map_accept_languages(&languages);
  let content_language = map_language_header(&languages);

  println!("Route for books from a user with the id {} in {:?}", id, languages);

  match service.get_by_user_id(id, &languages).await {
    Ok(items) => Ok((StatusCode::OK, content_language, Json(items))),
    Err(error) => Err(convert_service_error(error)),
  }
}
#[utoipa::path(get, path = "/{id}/books/{book_id}",
  responses(
    (status = 200, description = "User book based on the user id and book id", body = Option<UserBook>), ServerError, BadRequest,NotFound
  ),
  params(IdParam, AcceptLanguageParam,("book_id" = u32, Path,)),
  tag = "Users"
)]
async fn get_book_by_id(
  Path((user_id, book_id)): Path<(u32, u32)>,
  AcceptLanguageHeader(languages): AcceptLanguageHeader,
  State(app_state): State<AppState>,
) -> impl IntoResponse {
  let connection = app_state.pool.get().await.map_err(convert_error)?;
  let service = get_book_service(&connection);

  let languages = map_accept_languages(&languages);
  let content_language = map_language_header(&languages);

  println!(
    "Route for user book with the id {} from a user with the id {} in {:?}",
    book_id, user_id, languages
  );

  match service.get_by_book_id(user_id, book_id, &languages).await {
    Ok(item) => match item {
      None => Ok((StatusCode::NOT_FOUND, content_language, Json(None))),
      Some(item) => Ok((StatusCode::OK, content_language, Json(Some(item)))),
    },
    Err(error) => Err(convert_service_error(error)),
  }
}

//TODO Authorization
#[utoipa::path(post, path = "/{id}/books",
  responses(
    (status = 201, description = "Book association successfully added", body = UserBook), ServerError, BadRequest
  ),
  request_body(content = CreateUserBook),
  params(IdParam,AcceptLanguageParam),
  tag = "Users"
)]
async fn add_book(
  Path(id): Path<u32>,
  State(app_state): State<AppState>,
  AcceptLanguageHeader(languages): AcceptLanguageHeader,
  Json(create_user_book): Json<CreateUserBook>,
) -> impl IntoResponse {
  let mut connection = app_state.pool.get().await.map_err(convert_error)?;
  let transaction = connection.transaction().await.map_err(convert_error)?;
  let result = {
    let client = transaction.client();
    let service = get_mut_book_service(&transaction, client);

    let languages = map_accept_languages(&languages);
    let content_language = map_language_header(&languages);

    println!("Route for adding a book for a user with the id {id}");

    match service.add(id, create_user_book, &languages).await {
      Ok(item) => Ok((StatusCode::CREATED, content_language, Json(item))),
      Err(error) => Err(convert_service_error(error)),
    }
  };
  transaction.commit().await.map_err(convert_error)?;
  result
}
//TODO Authorization
#[utoipa::path(delete, path = "/{id}/books/{book_id}",
  responses(
    (status = 200, description = "Book association successfully removed"), ServerError, BadRequest
  ),
  params(IdParam, ("book_id" = u32, Path,)),
  tag = "Users"
)]
async fn remove_book(Path((id, book_id)): Path<(u32, u32)>, State(app_state): State<AppState>) -> impl IntoResponse {
  let mut connection = app_state.pool.get().await.map_err(convert_error)?;
  let transaction = connection.transaction().await.map_err(convert_error)?;
  let result = {
    let client = transaction.client();
    let service = get_mut_book_service(&transaction, client);

    println!("Route for removing a book with the id {book_id} for a user with the id {id}");

    match service.remove(id, &[book_id]).await {
      Ok(()) => Ok(StatusCode::OK),
      Err(error) => Err(convert_service_error(error)),
    }
  };
  transaction.commit().await.map_err(convert_error)?;
  result
}

fn get_service(client: &Client) -> impl UserService + '_ {
  let image_repository = Arc::new(get_image_repository(client));
  let repository = get_user_repository(client, image_repository);
  get_user_service(Arc::new(repository))
}

pub fn get_book_service(client: &Client) -> impl UserBookService + '_ {
  let image_repository = Arc::new(get_image_repository(client));
  let franchise_repository = Arc::new(get_franchise_repository(client));
  let genre_repository = Arc::new(get_genre_repository(client));
  let theme_repository = Arc::new(get_theme_repository(client));
  let role_repository = Arc::new(get_role_repository(client));
  let person_repository = Arc::new(get_person_repository(client, image_repository.clone()));
  let book_genre_repository = Arc::new(get_book_genre_repository(client, genre_repository));
  let book_theme_repository = Arc::new(get_book_theme_repository(client, theme_repository));
  let book_involved_repository = Arc::new(get_book_involved_repository(client, person_repository, role_repository));
  let character_repository = Arc::new(get_character_repository(client, image_repository.clone()));
  let book_character_repository = Arc::new(get_book_character_repository(client, character_repository));

  let book_repository = Arc::new(get_book_repository(
    client,
    image_repository.clone(),
    franchise_repository,
    book_genre_repository,
    book_theme_repository,
    book_involved_repository,
    book_character_repository,
  ));
  let repository = Arc::new(get_user_book_repository(client, book_repository));
  get_user_book_service(repository)
}
pub fn get_mut_book_service<'a>(transaction: &'a Transaction<'a>, client: &'a Client) -> impl MutUserBookService + 'a {
  let image_repository = Arc::new(get_image_repository(client));
  let franchise_repository = Arc::new(get_franchise_repository(client));
  let genre_repository = Arc::new(get_genre_repository(client));
  let theme_repository = Arc::new(get_theme_repository(client));
  let role_repository = Arc::new(get_role_repository(client));
  let person_repository = Arc::new(get_person_repository(client, image_repository.clone()));
  let book_genre_repository = Arc::new(get_book_genre_repository(client, genre_repository));
  let book_theme_repository = Arc::new(get_book_theme_repository(client, theme_repository));
  let book_involved_repository = Arc::new(get_book_involved_repository(client, person_repository, role_repository));
  let character_repository = Arc::new(get_character_repository(client, image_repository.clone()));
  let book_character_repository = Arc::new(get_book_character_repository(client, character_repository));

  let book_repository = Arc::new(get_book_repository(
    client,
    image_repository.clone(),
    franchise_repository,
    book_genre_repository,
    book_theme_repository,
    book_involved_repository,
    book_character_repository,
  ));
  let user_repository = Arc::new(get_user_repository(client, image_repository));
  let user_book_repository = Arc::new(get_user_book_repository(client, book_repository.clone()));
  let repository = Arc::new(get_mut_user_book_repository(transaction, user_book_repository.clone()));
  get_mut_user_book_service(user_repository, user_book_repository, repository, book_repository)
}
