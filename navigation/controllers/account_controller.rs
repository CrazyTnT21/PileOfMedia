use crate::openapi::responses::forbidden::Forbidden;
use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::{get, post};
use axum::{Json, Router, debug_handler};
use chrono::Utc;
use multipart::MultiPartRequest;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio_postgres::{Client, Transaction};
use utoipa::ToSchema;

use crate::app_state::AppState;
use crate::controllers::{convert_error, convert_service_error};
use crate::extractors::headers::authorization::JWTAuthorization;
use crate::implementations::{
  get_account_repository, get_account_service, get_file_repository, get_image_repository, get_mut_account_repository,
  get_mut_account_service, get_mut_file_repository, get_mut_file_service, get_mut_image_repository,
  get_mut_image_service, get_mut_user_repository, get_mut_user_service, get_user_repository,
};
use crate::jwt::{encode_token, parse_token};
use crate::openapi::responses::bad_request::BadRequest;
use crate::openapi::responses::not_authorized::NotAuthorized;
use crate::openapi::responses::server_error::ServerError;
use domain::entities::account::create_account::CreateAccount;
use domain::entities::account::{Email, Password};
use domain::entities::user::User;
use services::account_service::AccountService;
use services::account_service::mut_account_service::MutAccountService;

pub mod account_doc;

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct LoginData {
  email: String,
  password: String,
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct LoginReturnData {
  token: String,
  user: User,
}

pub fn routes(app_state: AppState) -> Router {
  Router::new()
    .route("/login", post(login))
    .route("/register", post(register))
    .route("/refresh", get(refresh_token))
    .with_state(app_state)
}

#[utoipa::path(post, path = "/register",
  responses(
    (status = 201, description = "Returned JWT and user. Valid for a week", body = LoginReturnData), ServerError, BadRequest
  ),
  request_body(content_type = ["multipart/form-data"], content = CreateAccount),
  tag = "Accounts"
)]
#[debug_handler]
async fn register(
  State(app_state): State<AppState>,
  MultiPartRequest(account): MultiPartRequest<CreateAccount>,
) -> Result<(StatusCode, Json<LoginReturnData>), (StatusCode, String)> {
  let mut connection = app_state.pool.get().await.map_err(convert_error)?;
  let transaction = connection.transaction().await.map_err(convert_error)?;

  let account = {
    let service = get_mut_service(&transaction, &app_state.display_path, &app_state.content_path);
    service.create(account).await.map_err(convert_service_error)?
  };
  transaction.commit().await.map_err(convert_error)?;

  let in_a_week = usize::try_from(Utc::now().timestamp()).unwrap() + 604_800;
  let token = encode_token(account.user.id, "Register".to_string(), in_a_week, &app_state.secret)?;
  let user = account.user;
  Ok((StatusCode::CREATED, Json(LoginReturnData { token, user })))
}

#[utoipa::path(post, path = "/login",
responses(
(status = 200, description = "Returned JWT. Valid for a week", body = LoginReturnData), ServerError, NotAuthorized
),
request_body = LoginData,
tag = "Accounts"
)]
async fn login(
  State(app_state): State<AppState>,
  Json(login_data): Json<LoginData>,
) -> Result<(StatusCode, Json<LoginReturnData>), (StatusCode, String)> {
  let pooled = app_state.pool.get().await.unwrap();

  let password = login_data.password;
  let email = login_data.email;

  let account = {
    let service = get_service(&pooled);
    service
      .login(&Email(email), &Password(password))
      .await
      .map_err(convert_service_error)?
  };

  let in_a_week = usize::try_from(Utc::now().timestamp()).unwrap() + 604_800;
  let token = encode_token(account.user.id, "Login".to_string(), in_a_week, &app_state.secret)?;
  let user = account.user;
  Ok((StatusCode::OK, Json(LoginReturnData { token, user })))
}

#[utoipa::path(get, path = "/refresh",
  responses(
    (status = 200, description = "Returned JWT. Valid for an hour", body = String), ServerError, Forbidden
  ),
  security(("user_token" = [])),
  tag = "Accounts"
)]
async fn refresh_token(
  auth: JWTAuthorization,
  State(app_state): State<AppState>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
  let token = parse_token(auth, &app_state.secret)?;
  let in_one_hour = usize::try_from(Utc::now().timestamp()).unwrap() + 3600;
  let token = encode_token(token.user_id, "Refresh".to_string(), in_one_hour, &app_state.secret)?;
  Ok((StatusCode::OK, token))
}

fn get_mut_service<'a>(
  transaction: &'a Transaction,
  display_path: &'a str,
  path: &'a str,
) -> impl MutAccountService + 'a {
  let image_repository = Arc::new(get_image_repository(transaction.client()));
  let user_repository = Arc::new(get_user_repository(transaction.client(), image_repository.clone()));
  let account_repository = Arc::new(get_account_repository(transaction.client(), user_repository.clone()));
  let mut_account_repository = Arc::new(get_mut_account_repository(transaction, account_repository.clone()));
  let mut_user_repository = Arc::new(get_mut_user_repository(transaction, user_repository));
  let mut_file_repository = Arc::new(get_mut_file_repository());
  let file_repository = Arc::new(get_file_repository());
  let mut_image_repository = Arc::new(get_mut_image_repository(
    transaction,
    image_repository,
    mut_file_repository.clone(),
    file_repository,
  ));
  let mut_file_service = Arc::new(get_mut_file_service(mut_file_repository));
  //TODO: Make configurable
  let mut_image_service = Arc::new(get_mut_image_service(
    mut_image_repository,
    mut_file_service,
    display_path,
    path,
  ));
  let mut_user_service = Arc::new(get_mut_user_service(mut_user_repository, mut_image_service));
  let account_service = Arc::new(get_account_service(account_repository));
  get_mut_account_service(mut_account_repository, account_service, mut_user_service)
}

fn get_service(client: &Client) -> impl AccountService + '_ {
  let image_repository = Arc::new(get_image_repository(client));
  let user_repository = Arc::new(get_user_repository(client, image_repository));
  let account_repository = Arc::new(get_account_repository(client, user_repository));
  get_account_service(account_repository)
}
