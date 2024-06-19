use std::sync::Arc;

use axum::{Json, Router};
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{delete, get, post};
use tokio_postgres::{Client, Transaction};

use services::book_service::book_character_service::BookCharacterService;
use services::book_service::book_character_service::mut_book_character_service::MutBookCharacterService;
use services::book_service::book_genre_service::BookGenreService;
use services::book_service::book_involved_service::BookInvolvedService;
use services::book_service::book_theme_service::BookThemeService;
use services::book_service::BookService;

use crate::app_state::AppState;
use crate::controllers::{append_content_language_header, content_language_header, convert_error, convert_service_error, DEFAULT_LANGUAGE, get_language, set_pagination_limit};
use crate::extractors::headers::accept_language::AcceptLanguageHeader;
use crate::extractors::query_pagination::QueryPagination;
use crate::implementations::{get_book_character_repository, get_book_character_service, get_book_genre_repository, get_book_genre_service, get_book_involved_repository, get_book_involved_service, get_book_repository, get_book_service, get_book_theme_repository, get_book_theme_service, get_character_repository, get_genre_repository, get_image_repository, get_mut_book_character_repository, get_mut_book_character_service, get_person_repository, get_role_repository, get_theme_repository};
use crate::openapi::params::header::accept_language::AcceptLanguageParam;
use crate::openapi::params::path::id::IdParam;
use crate::openapi::params::path::title::TitleParam;
use crate::openapi::params::query::count::CountParam;
use crate::openapi::params::query::page::PageParam;
use crate::openapi::responses::bad_request::BadRequest;
use crate::openapi::responses::not_found::NotFound;
use crate::openapi::responses::server_error::ServerError;

pub mod book_doc;

pub fn routes(app_state: AppState) -> Router {
  Router::new()
    .route("/", get(get_items))
    .route("/:id", get(get_by_id))
    .route("/title/:title", get(get_by_title))
    .route("/:ids/genres", get(get_genres))
    .route("/:id/themes", get(get_themes))
    .route("/:id/characters", get(get_characters))
    .route("/:id/characters/:character_id", post(add_character))
    .route("/:id/characters/:character_id", delete(remove_character))
    .route("/:id/involved", get(get_involved))
    .with_state(app_state)
}

#[utoipa::path(get, path = "",
  responses(
    (status = 200, description = "Returned books", body = BooksTotal), ServerError, BadRequest),
  params(AcceptLanguageParam, PageParam, CountParam),
  tag = "Books"
)]
async fn get_items(AcceptLanguageHeader(languages): AcceptLanguageHeader, State(app_state): State<AppState>, Query(mut pagination): Query<QueryPagination>) -> impl IntoResponse {
  let connection = app_state.pool.get().await.map_err(convert_error)?;
  let service = get_service(&connection);

  let language = get_language(languages, DEFAULT_LANGUAGE);
  set_pagination_limit(&mut pagination);

  println!("Route for books in {}", language);

  let mut content_language = content_language_header(language);
  append_content_language_header(&mut content_language, DEFAULT_LANGUAGE);

  match service.get(language, pagination.into()).await {
    Ok(books) => Ok((StatusCode::OK, content_language, Json(books))),
    Err(error) => Err(convert_service_error(error))
  }
}

#[utoipa::path(get, path = "/{id}",
  responses(
    (status = 200, description = "Returned book based on the id", body = Book), ServerError, BadRequest, NotFound
  ),
  params(IdParam, AcceptLanguageParam),
  tag = "Books"
)]
async fn get_by_id(Path(id): Path<u32>, AcceptLanguageHeader(languages): AcceptLanguageHeader, State(app_state): State<AppState>) -> impl IntoResponse {
  let connection = app_state.pool.get().await.map_err(convert_error)?;
  let service = get_service(&connection);

  let language = get_language(languages, DEFAULT_LANGUAGE);

  println!("Route for a book with id {} in {}", id, language);

  let mut content_language = content_language_header(language);
  append_content_language_header(&mut content_language, DEFAULT_LANGUAGE);

  match service.get_by_id(id, language).await {
    Ok(item) => Ok((StatusCode::OK, content_language, Json(item))),
    Err(error) => Err(convert_service_error(error))
  }
}

#[utoipa::path(get, path = "/title/{title}",
  responses(
    (status = 200, description = "Returned books based on the title", body = BooksTotal), ServerError, BadRequest
  ),
  params(TitleParam, AcceptLanguageParam, PageParam, CountParam),
  tag = "Books"
)]
async fn get_by_title(Path(title): Path<String>, AcceptLanguageHeader(languages): AcceptLanguageHeader, State(app_state): State<AppState>, Query(mut pagination): Query<QueryPagination>) -> impl IntoResponse {
  let connection = app_state.pool.get().await.map_err(convert_error)?;
  let service = get_service(&connection);

  let language = get_language(languages, DEFAULT_LANGUAGE);
  set_pagination_limit(&mut pagination);

  println!("Route for books with the title {} in {}", title, language);

  let mut content_language = content_language_header(language);
  append_content_language_header(&mut content_language, DEFAULT_LANGUAGE);

  match service.get_by_title(&title, language, pagination.into()).await {
    Ok(items) => Ok((StatusCode::OK, content_language, Json(items))),
    Err(error) => Err(convert_service_error(error))
  }
}

#[utoipa::path(get, path = "/{id}/genres",
  responses(
    (status = 200, description = "Returned genres based on the book id", body = GenresTotal), ServerError, BadRequest
  ),
  params(IdParam, AcceptLanguageParam, PageParam, CountParam),
  tag = "Books"
)]
async fn get_genres(Path(id): Path<u32>, AcceptLanguageHeader(languages): AcceptLanguageHeader, State(app_state): State<AppState>, Query(mut pagination): Query<QueryPagination>) -> impl IntoResponse {
  let connection = app_state.pool.get().await.map_err(convert_error)?;
  let service = get_genre_service(&connection);

  let language = get_language(languages, DEFAULT_LANGUAGE);
  set_pagination_limit(&mut pagination);

  println!("Route for genres from a book with the id {} in {}", id, language);

  let mut content_language = content_language_header(language);
  append_content_language_header(&mut content_language, DEFAULT_LANGUAGE);

  match service.get(id, language, pagination.into()).await {
    Ok(items) => Ok((StatusCode::OK, content_language, Json(items))),
    Err(error) => Err(convert_service_error(error))
  }
}

#[utoipa::path(get, path = "/{id}/themes",
  responses(
    (status = 200, description = "Returned themes based on the book id", body = ThemesTotal), ServerError, BadRequest
  ),
  params(IdParam, AcceptLanguageParam, PageParam, CountParam),
  tag = "Books"
)]
async fn get_themes(Path(id): Path<u32>, AcceptLanguageHeader(languages): AcceptLanguageHeader, State(app_state): State<AppState>, Query(mut pagination): Query<QueryPagination>) -> impl IntoResponse {
  let connection = app_state.pool.get().await.map_err(convert_error)?;
  let service = get_theme_service(&connection);

  let language = get_language(languages, DEFAULT_LANGUAGE);
  set_pagination_limit(&mut pagination);

  println!("Route for themes from a book with the id {} in {}", id, language);

  let mut content_language = content_language_header(language);
  append_content_language_header(&mut content_language, DEFAULT_LANGUAGE);

  match service.get(id, language, pagination.into()).await {
    Ok(items) => Ok((StatusCode::OK, content_language, Json(items))),
    Err(error) => Err(convert_service_error(error))
  }
}

#[utoipa::path(get, path = "/{id}/characters",
  responses(
    (status = 200, description = "Returned characters based on the book id", body = BookCharactersTotal), ServerError, BadRequest
  ),
  params(IdParam, AcceptLanguageParam, PageParam, CountParam),
  tag = "Books"
)]
async fn get_characters(Path(id): Path<u32>, AcceptLanguageHeader(languages): AcceptLanguageHeader, State(app_state): State<AppState>, Query(mut pagination): Query<QueryPagination>) -> impl IntoResponse {
  let connection = app_state.pool.get().await.map_err(convert_error)?;
  let service = get_character_service(&connection);

  let language = get_language(languages, DEFAULT_LANGUAGE);
  set_pagination_limit(&mut pagination);

  println!("Route for characters from a book with the id {} in {}", id, language);

  let mut content_language = content_language_header(language);
  append_content_language_header(&mut content_language, DEFAULT_LANGUAGE);

  match service.get(id, language, pagination.into()).await {
    Ok(items) => Ok((StatusCode::OK, content_language, Json(items))),
    Err(error) => Err(convert_service_error(error))
  }
}

#[utoipa::path(get, path = "/{id}/involved",
  responses(
    (status = 200, description = "Returned people involved based on the book id", body = BookInvolvedTotal), ServerError, BadRequest
  ),
  params(IdParam, AcceptLanguageParam, PageParam, CountParam),
  tag = "Books"
)]
async fn get_involved(Path(id): Path<u32>, AcceptLanguageHeader(languages): AcceptLanguageHeader, State(app_state): State<AppState>, Query(mut pagination): Query<QueryPagination>) -> impl IntoResponse {
  let connection = app_state.pool.get().await.map_err(convert_error)?;
  let service = get_involved_service(&connection);

  let language = get_language(languages, DEFAULT_LANGUAGE);
  set_pagination_limit(&mut pagination);

  println!("Route for people involved from a book with the id {} in {}", id, language);

  let mut content_language = content_language_header(language);
  append_content_language_header(&mut content_language, DEFAULT_LANGUAGE);

  match service.get(id, language, pagination.into()).await {
    Ok(items) => Ok((StatusCode::OK, content_language, Json(items))),
    Err(error) => Err(convert_service_error(error))
  }
}

#[utoipa::path(post, path = "/{id}/characters/{character_id}",
  responses(
    (status = 200, description = "Character association successfully added"), ServerError, BadRequest
  ),
  params(IdParam, ("character_id" = u32, Path,)),
  tag = "Books"
)]
async fn add_character(Path((id, character_id)): Path<(u32, u32)>, State(app_state): State<AppState>) -> impl IntoResponse {
  let mut connection = app_state.pool.get().await.map_err(convert_error)?;
  let transaction = connection.transaction().await.map_err(convert_error)?;
  let result = {
    let client = transaction.client();
    let service = get_mut_character_service(&transaction, client);

    println!("Route for adding a character with the id {character_id} for a book with the id {id}");

    match service.add(id, &[character_id]).await {
      Ok(_) => Ok(StatusCode::OK),
      Err(error) => Err(convert_service_error(error))
    }
  };
  transaction.commit().await.map_err(convert_error)?;
  result
}

#[utoipa::path(delete, path = "/{id}/characters/{character_id}",
  responses(
    (status = 200, description = "Character association successfully removed"), ServerError, BadRequest
  ),
  params(IdParam, ("character_id" = u32, Path,)),
  tag = "Books"
)]
async fn remove_character(Path((id, character_id)): Path<(u32, u32)>, State(app_state): State<AppState>) -> impl IntoResponse {
  let mut connection = app_state.pool.get().await.map_err(convert_error)?;
  let transaction = connection.transaction().await.map_err(convert_error)?;
  let result = {
    let client = transaction.client();
    let service = get_mut_character_service(&transaction, client);

    println!("Route for removing a character with the id {character_id} for a book with the id {id}");

    match service.remove(id, &[character_id]).await {
      Ok(_) => Ok(StatusCode::OK),
      Err(error) => Err(convert_service_error(error))
    }
  };
  transaction.commit().await.map_err(convert_error)?;
  result
}

fn get_genre_service(connection: &Client) -> impl BookGenreService + '_ {
  let image_repository = Arc::new(get_image_repository(connection));
  let book_repository = Arc::new(get_book_repository(connection, DEFAULT_LANGUAGE, image_repository.clone()));
  let genre_repository = Arc::new(get_genre_repository(connection, DEFAULT_LANGUAGE));
  let repository = Arc::new(get_book_genre_repository(connection, DEFAULT_LANGUAGE, book_repository, genre_repository));
  get_book_genre_service(repository)
}

fn get_theme_service(connection: &Client) -> impl BookThemeService + '_ {
  let image_repository = Arc::new(get_image_repository(connection));
  let book_repository = Arc::new(get_book_repository(connection, DEFAULT_LANGUAGE, image_repository.clone()));
  let theme_repository = Arc::new(get_theme_repository(connection, DEFAULT_LANGUAGE));
  let repository = Arc::new(get_book_theme_repository(connection, DEFAULT_LANGUAGE, book_repository, theme_repository));
  get_book_theme_service(repository)
}

fn get_character_service(connection: &Client) -> impl BookCharacterService + '_ {
  let image_repository = Arc::new(get_image_repository(connection));
  let book_repository = Arc::new(get_book_repository(connection, DEFAULT_LANGUAGE, image_repository.clone()));
  let character_repository = Arc::new(get_character_repository(connection, DEFAULT_LANGUAGE, image_repository));
  let repository = Arc::new(get_book_character_repository(connection, DEFAULT_LANGUAGE, book_repository, character_repository));
  get_book_character_service(repository)
}

fn get_mut_character_service<'a>(transaction: &'a Transaction<'a>, client: &'a Client) -> impl MutBookCharacterService + 'a {
  let image_repository = Arc::new(get_image_repository(client));
  let book_repository = Arc::new(get_book_repository(client, DEFAULT_LANGUAGE, image_repository.clone()));
  let character_repository = Arc::new(get_character_repository(client, DEFAULT_LANGUAGE, image_repository));
  let book_character_repository = get_book_character_repository(client, DEFAULT_LANGUAGE, book_repository.clone(), character_repository.clone());
  let repository = get_mut_book_character_repository(transaction);
  get_mut_book_character_service(book_repository, Arc::new(book_character_repository), Arc::new(repository), character_repository)
}

fn get_involved_service(connection: &Client) -> impl BookInvolvedService + '_ {
  let image_repository = Arc::new(get_image_repository(connection));
  let book_repository = Arc::new(get_book_repository(connection, DEFAULT_LANGUAGE, image_repository.clone()));
  let person_repository = Arc::new(get_person_repository(connection, DEFAULT_LANGUAGE, image_repository.clone()));
  let role_repository = Arc::new(get_role_repository(connection, DEFAULT_LANGUAGE));
  let repository = Arc::new(get_book_involved_repository(connection, DEFAULT_LANGUAGE, book_repository, person_repository, role_repository));
  get_book_involved_service(repository)
}

fn get_service(connection: &Client) -> impl BookService + '_ {
  let image_repository = Arc::new(get_image_repository(connection));
  let repository = get_book_repository(connection, DEFAULT_LANGUAGE, image_repository);
  get_book_service(Arc::new(repository))
}
