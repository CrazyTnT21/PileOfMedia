use crate::app_state::AppState;
use crate::controllers::book_controller::book_implementations::{
  get_character_service, get_genre_service, get_involved_service, get_mut_character_service, get_mut_genre_service,
  get_mut_involved_service, get_mut_service, get_mut_theme_service, get_service, get_theme_service,
};
use crate::controllers::{
  convert_error, convert_service_error, map_accept_languages, map_language_header, set_pagination_limit,
};
use crate::extractors::headers::accept_language::AcceptLanguageHeader;
use crate::extractors::query_pagination::QueryPagination;
use crate::openapi::params::header::accept_language::AcceptLanguageParam;
use crate::openapi::params::path::id::IdParam;
use crate::openapi::params::path::slug::SlugParam;
use crate::openapi::params::path::title::TitleParam;
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
use domain::entities::book::create_book::CreateBook;
use domain::entities::involved::InvolvedId;
use domain::slug::Slug;
use multipart::MultiPartRequest;
use services::book_service::book_character_service::BookCharacterService;
use services::book_service::book_character_service::mut_book_character_service::MutBookCharacterService;
use services::book_service::book_genre_service::BookGenreService;
use services::book_service::book_genre_service::mut_book_genre_service::MutBookGenreService;
use services::book_service::book_involved_service::BookInvolvedService;
use services::book_service::book_involved_service::mut_book_involved_service::MutBookInvolvedService;
use services::book_service::book_theme_service::BookThemeService;
use services::book_service::book_theme_service::mut_book_theme_service::MutBookThemeService;
use services::book_service::mut_book_service::MutBookService;
use services::book_service::{BookService, BookServiceError};
use services::traits::service_error::ServiceError;

pub mod book_doc;
mod book_implementations;

pub fn routes(app_state: AppState) -> Router {
  Router::new()
    .route("/", get(get_items))
    .route("/", post(create_book))
    .route("/{id}", get(get_by_id))
    .route("/{id}", delete(delete_book))
    .route("/{id}/statistic", get(get_statistic))
    .route("/title/{title}", get(get_by_title))
    .route("/slug/{slug}", get(get_by_slug))
    .route("/{id}/genres", get(get_genres))
    .route("/{id}/genres/{genre_id}", post(add_genre))
    .route("/{id}/genres/{genre_id}", delete(remove_genre))
    .route("/{id}/themes", get(get_themes))
    .route("/{id}/themes/{theme_id}", post(add_theme))
    .route("/{id}/themes/{theme_id}", delete(remove_theme))
    .route("/{id}/characters", get(get_characters))
    .route("/{id}/characters/{character_id}", post(add_character))
    .route("/{id}/characters/{character_id}", delete(remove_character))
    .route("/{id}/involved", get(get_involved))
    .route("/{id}/involved/{person_id}/{role_id}", post(add_involved))
    .route("/{id}/involved/{person_id}/{role_id}", delete(remove_involved))
    .with_state(app_state)
}

#[utoipa::path(get, path = "",
  responses(
    (status = 200, description = "Returned books", body = BooksTotal), ServerError, BadRequest),
  params(AcceptLanguageParam, PageParam, CountParam),
  tag = "Books"
)]
async fn get_items(
  AcceptLanguageHeader(languages): AcceptLanguageHeader,
  State(app_state): State<AppState>,
  Query(mut pagination): Query<QueryPagination>,
) -> impl IntoResponse {
  let connection = app_state.pool.get().await.map_err(convert_error)?;
  let service = get_service(&connection);

  let languages = map_accept_languages(&languages);
  let content_language = map_language_header(&languages);
  set_pagination_limit(&mut pagination);

  println!("Route for books in {:?}", languages);

  match service.get(&languages, pagination.into()).await {
    Ok(books) => Ok((StatusCode::OK, content_language, Json(books))),
    Err(error) => Err(convert_service_error(error)),
  }
}

#[utoipa::path(get, path = "/{id}",
  responses(
    (status = 200, description = "Returned book based on the id", body = Book), ServerError, BadRequest, NotFound
  ),
  params(IdParam, AcceptLanguageParam),
  tag = "Books"
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

  println!("Route for a book with id {} in {:?}", id, languages);

  match service.get_by_id(id, &languages).await {
    Ok(Some(item)) => Ok((StatusCode::OK, content_language, Json(item))),
    Ok(None) => Err((StatusCode::NOT_FOUND, "".to_string())),
    Err(error) => Err(convert_service_error(error)),
  }
}
#[utoipa::path(get, path = "/{id}/statistic",
  responses(
    (status = 200, description = "Returned book statistic based on the id", body = BookStatistic), ServerError, BadRequest, NotFound
  ),
  params(IdParam),
  tag = "Books"
)]
async fn get_statistic(Path(id): Path<u32>, State(app_state): State<AppState>) -> impl IntoResponse {
  let connection = app_state.pool.get().await.map_err(convert_error)?;
  let service = get_service(&connection);

  println!("Route for a book statistic with id {}", id);

  match service.get_statistics(&[id]).await {
    Ok(mut items) => Ok((StatusCode::OK, Json(items.remove(&id).unwrap()))),
    Err(error) => Err(match error {
      ServiceError::ClientError(error) => match error {
        BookServiceError::NonExistentBooks(_) => (StatusCode::NOT_FOUND, error.to_string()),
        BookServiceError::OtherError(_) => (StatusCode::BAD_REQUEST, error.to_string()),
      },
      ServiceError::ServerError(_) => convert_service_error(error),
    }),
  }
}

#[utoipa::path(get, path = "/title/{title}",
  responses(
    (status = 200, description = "Returned books based on the title", body = BooksTotal), ServerError, BadRequest
  ),
  params(TitleParam, AcceptLanguageParam, PageParam, CountParam),
  tag = "Books"
)]
async fn get_by_title(
  Path(title): Path<String>,
  AcceptLanguageHeader(languages): AcceptLanguageHeader,
  State(app_state): State<AppState>,
  Query(mut pagination): Query<QueryPagination>,
) -> impl IntoResponse {
  let connection = app_state.pool.get().await.map_err(convert_error)?;
  let service = get_service(&connection);

  let languages = map_accept_languages(&languages);
  let content_language = map_language_header(&languages);
  set_pagination_limit(&mut pagination);

  println!("Route for books with the title {} in {:?}", title, languages);

  match service.get_by_title(&title, &languages, pagination.into()).await {
    Ok(items) => Ok((StatusCode::OK, content_language, Json(items))),
    Err(error) => Err(convert_service_error(error)),
  }
}
#[utoipa::path(get, path = "/slug/{slug}",
  responses(
    (status = 200, description = "Returned book based on the slug", body = Book), ServerError, BadRequest, NotFound
  ),
  params(SlugParam, AcceptLanguageParam),
  tag = "Books"
)]
async fn get_by_slug(
  Path(slug): Path<Slug>,
  AcceptLanguageHeader(languages): AcceptLanguageHeader,
  State(app_state): State<AppState>,
) -> impl IntoResponse {
  let connection = app_state.pool.get().await.map_err(convert_error)?;
  let service = get_service(&connection);

  let languages = map_accept_languages(&languages);
  let content_language = map_language_header(&languages);

  println!("Route for a book with slug {} in {:?}", slug, languages);

  match service.get_by_slug(&slug, &languages).await {
    Ok(Some(item)) => Ok((StatusCode::OK, content_language, Json(item))),
    Ok(None) => Err((StatusCode::NOT_FOUND, "".to_string())),
    Err(error) => Err(convert_service_error(error)),
  }
}

#[utoipa::path(get, path = "/{id}/genres",
  responses(
    (status = 200, description = "Returned genres based on the book id", body = GenresTotal), ServerError, BadRequest
  ),
  params(IdParam, AcceptLanguageParam),
  tag = "Books"
)]
async fn get_genres(
  Path(id): Path<u32>,
  AcceptLanguageHeader(languages): AcceptLanguageHeader,
  State(app_state): State<AppState>,
) -> impl IntoResponse {
  let connection = app_state.pool.get().await.map_err(convert_error)?;
  let service = get_genre_service(&connection);

  let languages = map_accept_languages(&languages);
  let content_language = map_language_header(&languages);

  println!("Route for genres from a book with the id {} in {:?}", id, languages);

  match service.get_by_id(id, &languages).await {
    Ok(items) => Ok((StatusCode::OK, content_language, Json(items))),
    Err(error) => Err(convert_service_error(error)),
  }
}

#[utoipa::path(get, path = "/{id}/themes",
  responses(
    (status = 200, description = "Returned themes based on the book id", body = ThemesTotal), ServerError, BadRequest
  ),
  params(IdParam, AcceptLanguageParam),
  tag = "Books"
)]
async fn get_themes(
  Path(id): Path<u32>,
  AcceptLanguageHeader(languages): AcceptLanguageHeader,
  State(app_state): State<AppState>,
) -> impl IntoResponse {
  let connection = app_state.pool.get().await.map_err(convert_error)?;
  let service = get_theme_service(&connection);

  let languages = map_accept_languages(&languages);
  let content_language = map_language_header(&languages);

  println!("Route for themes from a book with the id {} in {:?}", id, languages);

  match service.get_by_id(id, &languages).await {
    Ok(items) => Ok((StatusCode::OK, content_language, Json(items))),
    Err(error) => Err(convert_service_error(error)),
  }
}

#[utoipa::path(get, path = "/{id}/characters",
  responses(
    (status = 200, description = "Returned characters based on the book id", body = Vec < BookCharacter >), ServerError, BadRequest
  ),
  params(IdParam, AcceptLanguageParam),
  tag = "Books"
)]
async fn get_characters(
  Path(id): Path<u32>,
  AcceptLanguageHeader(languages): AcceptLanguageHeader,
  State(app_state): State<AppState>,
) -> impl IntoResponse {
  let connection = app_state.pool.get().await.map_err(convert_error)?;
  let service = get_character_service(&connection);

  let languages = map_accept_languages(&languages);
  let content_language = map_language_header(&languages);

  println!("Route for characters from a book with the id {} in {:?}", id, languages);

  match service.get_by_id(id, &languages).await {
    Ok(items) => Ok((StatusCode::OK, content_language, Json(items))),
    Err(error) => Err(convert_service_error(error)),
  }
}

#[utoipa::path(get, path = "/{id}/involved",
  responses(
    (status = 200, description = "Returned people involved based on the book id", body = Vec < Involved >), ServerError, BadRequest
  ),
  params(IdParam, AcceptLanguageParam),
  tag = "Books"
)]
async fn get_involved(
  Path(id): Path<u32>,
  AcceptLanguageHeader(languages): AcceptLanguageHeader,
  State(app_state): State<AppState>,
) -> impl IntoResponse {
  let connection = app_state.pool.get().await.map_err(convert_error)?;
  let service = get_involved_service(&connection);

  let languages = map_accept_languages(&languages);
  let content_language = map_language_header(&languages);

  println!(
    "Route for people involved from a book with the id {} in {:?}",
    id, languages
  );

  match service.get_by_id(id, &languages).await {
    Ok(items) => Ok((StatusCode::OK, content_language, Json(items))),
    Err(error) => Err(convert_service_error(error)),
  }
}

#[utoipa::path(post, path = "/{id}/characters/{character_id}",
  responses(
    (status = 200, description = "Character association successfully added"), ServerError, BadRequest
  ),
  params(IdParam, ("character_id" = u32, Path,)),
  tag = "Books"
)]
async fn add_character(
  Path((id, character_id)): Path<(u32, u32)>,
  State(app_state): State<AppState>,
) -> impl IntoResponse {
  let mut connection = app_state.pool.get().await.map_err(convert_error)?;
  let transaction = connection.transaction().await.map_err(convert_error)?;
  let result = {
    let client = transaction.client();
    let service = get_mut_character_service(&transaction, client);

    println!("Route for adding a character with the id {character_id} for a book with the id {id}");

    match service.add(id, &[character_id]).await {
      Ok(()) => Ok(StatusCode::OK),
      Err(error) => Err(convert_service_error(error)),
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
async fn remove_character(
  Path((id, character_id)): Path<(u32, u32)>,
  State(app_state): State<AppState>,
) -> impl IntoResponse {
  let mut connection = app_state.pool.get().await.map_err(convert_error)?;
  let transaction = connection.transaction().await.map_err(convert_error)?;
  let result = {
    let client = transaction.client();
    let service = get_mut_character_service(&transaction, client);

    println!("Route for removing a character with the id {character_id} for a book with the id {id}");

    match service.remove(id, &[character_id]).await {
      Ok(()) => Ok(StatusCode::OK),
      Err(error) => Err(convert_service_error(error)),
    }
  };
  transaction.commit().await.map_err(convert_error)?;
  result
}

#[utoipa::path(post, path = "/{id}/genres/{genre_id}",
  responses(
    (status = 200, description = "Genre association successfully added"), ServerError, BadRequest
  ),
  params(IdParam, ("genre_id" = u32, Path,)),
  tag = "Books"
)]
async fn add_genre(Path((id, genre_id)): Path<(u32, u32)>, State(app_state): State<AppState>) -> impl IntoResponse {
  let mut connection = app_state.pool.get().await.map_err(convert_error)?;
  let transaction = connection.transaction().await.map_err(convert_error)?;
  let result = {
    let client = transaction.client();
    let service = get_mut_genre_service(&transaction, client);

    println!("Route for adding a genre with the id {genre_id} for a book with the id {id}");

    match service.add(id, &[genre_id]).await {
      Ok(()) => Ok(StatusCode::OK),
      Err(error) => Err(convert_service_error(error)),
    }
  };
  transaction.commit().await.map_err(convert_error)?;
  result
}

#[utoipa::path(delete, path = "/{id}/genres/{genre_id}",
  responses(
    (status = 200, description = "Genre association successfully removed"), ServerError, BadRequest
  ),
  params(IdParam, ("genre_id" = u32, Path,)),
  tag = "Books"
)]
async fn remove_genre(Path((id, genre_id)): Path<(u32, u32)>, State(app_state): State<AppState>) -> impl IntoResponse {
  let mut connection = app_state.pool.get().await.map_err(convert_error)?;
  let transaction = connection.transaction().await.map_err(convert_error)?;
  let result = {
    let client = transaction.client();
    let service = get_mut_genre_service(&transaction, client);

    println!("Route for removing a genre with the id {genre_id} for a book with the id {id}");

    match service.remove(id, &[genre_id]).await {
      Ok(()) => Ok(StatusCode::OK),
      Err(error) => Err(convert_service_error(error)),
    }
  };
  transaction.commit().await.map_err(convert_error)?;
  result
}

#[utoipa::path(post, path = "/{id}/themes/{theme_id}",
  responses(
    (status = 200, description = "Theme association successfully added"), ServerError, BadRequest
  ),
  params(IdParam, ("theme_id" = u32, Path,)),
  tag = "Books"
)]
async fn add_theme(Path((id, theme_id)): Path<(u32, u32)>, State(app_state): State<AppState>) -> impl IntoResponse {
  let mut connection = app_state.pool.get().await.map_err(convert_error)?;
  let transaction = connection.transaction().await.map_err(convert_error)?;
  let result = {
    let client = transaction.client();
    let service = get_mut_theme_service(&transaction, client);

    println!("Route for adding a theme with the id {theme_id} for a book with the id {id}");

    match service.add(id, &[theme_id]).await {
      Ok(()) => Ok(StatusCode::OK),
      Err(error) => Err(convert_service_error(error)),
    }
  };
  transaction.commit().await.map_err(convert_error)?;
  result
}

#[utoipa::path(delete, path = "/{id}/themes/{theme_id}",
  responses(
    (status = 200, description = "Theme association successfully removed"), ServerError, BadRequest
  ),
  params(IdParam, ("theme_id" = u32, Path,)),
  tag = "Books"
)]
async fn remove_theme(Path((id, theme_id)): Path<(u32, u32)>, State(app_state): State<AppState>) -> impl IntoResponse {
  let mut connection = app_state.pool.get().await.map_err(convert_error)?;
  let transaction = connection.transaction().await.map_err(convert_error)?;
  let result = {
    let client = transaction.client();
    let service = get_mut_theme_service(&transaction, client);

    println!("Route for removing a theme with the id {theme_id} for a book with the id {id}");

    match service.remove(id, &[theme_id]).await {
      Ok(()) => Ok(StatusCode::OK),
      Err(error) => Err(convert_service_error(error)),
    }
  };
  transaction.commit().await.map_err(convert_error)?;
  result
}

#[utoipa::path(post, path = "/{id}/involved/{person_id}/{role_id}",
  responses(
    (status = 200, description = "Involved association successfully added"), ServerError, BadRequest
  ),
  params(IdParam, ("person_id" = u32, Path,), ("role_id" = u32, Path,)),
  tag = "Books"
)]
async fn add_involved(
  Path((id, person_id, role_id)): Path<(u32, u32, u32)>,
  State(app_state): State<AppState>,
) -> impl IntoResponse {
  let mut connection = app_state.pool.get().await.map_err(convert_error)?;
  let transaction = connection.transaction().await.map_err(convert_error)?;
  let result = {
    let client = transaction.client();
    let service = get_mut_involved_service(&transaction, client);
    let involved_id = InvolvedId { person_id, role_id };
    println!("Route for adding an association with the ids {involved_id} for a book with the id {id}");

    match service.add(id, &[involved_id]).await {
      Ok(()) => Ok(StatusCode::OK),
      Err(error) => Err(convert_service_error(error)),
    }
  };
  transaction.commit().await.map_err(convert_error)?;
  result
}

#[utoipa::path(delete, path = "/{id}/involved/{person_id}/{role_id}",
  responses(
    (status = 200, description = "Involved association successfully removed"), ServerError, BadRequest
  ),
  params(IdParam, ("person_id" = u32, Path,), ("role_id" = u32, Path,)),
  tag = "Books"
)]
async fn remove_involved(
  Path((id, person_id, role_id)): Path<(u32, u32, u32)>,
  State(app_state): State<AppState>,
) -> impl IntoResponse {
  let mut connection = app_state.pool.get().await.map_err(convert_error)?;
  let transaction = connection.transaction().await.map_err(convert_error)?;
  let result = {
    let client = transaction.client();
    let service = get_mut_involved_service(&transaction, client);

    let involved_id = InvolvedId { person_id, role_id };
    println!("Route for removing an association with the ids {involved_id} for a book with the id {id}");

    match service.remove(id, &[involved_id]).await {
      Ok(()) => Ok(StatusCode::OK),
      Err(error) => Err(convert_service_error(error)),
    }
  };
  transaction.commit().await.map_err(convert_error)?;
  result
}
#[utoipa::path(post, path = "",
  responses(
    (status = 201, description = "Book successfully created", body = Book), ServerError, BadRequest
  ),
  request_body(content_type = ["multipart/form-data"], content = CreateBook),
  tag = "Books"
)]
async fn create_book(
  State(app_state): State<AppState>,
  MultiPartRequest(create_book): MultiPartRequest<CreateBook>,
) -> impl IntoResponse {
  let mut connection = app_state.pool.get().await.map_err(convert_error)?;
  let transaction = connection.transaction().await.map_err(convert_error)?;
  let result = {
    let client = transaction.client();
    let service = get_mut_service(&transaction, client, &app_state.display_path, &app_state.content_path);

    println!("Route for creating a book");

    match service.create(create_book).await {
      Ok(book) => Ok((StatusCode::CREATED, Json(book))),
      Err(error) => Err(convert_service_error(error)),
    }
  };
  transaction.commit().await.map_err(convert_error)?;
  result
}

#[utoipa::path(delete, path = "/{id}",
  responses(
    (status = 204, description = "Book successfully deleted"), ServerError, BadRequest
  ),
  params(("id" = u32, Path, description = "Id of the item to delete")),
  tag = "Books"
)]
async fn delete_book(Path(id): Path<u32>, State(app_state): State<AppState>) -> impl IntoResponse {
  let mut connection = app_state.pool.get().await.map_err(convert_error)?;
  let transaction = connection.transaction().await.map_err(convert_error)?;
  let result = {
    let client = transaction.client();
    let service = get_mut_service(&transaction, client, &app_state.display_path, &app_state.content_path);

    println!("Route for deleting a book");

    match service.delete(&[id]).await {
      Ok(()) => Ok(StatusCode::NO_CONTENT),
      Err(error) => Err(convert_service_error(error)),
    }
  };
  transaction.commit().await.map_err(convert_error)?;
  result
}
