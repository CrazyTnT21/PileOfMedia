use multipart::MultiPartRequest;
use crate::openapi::responses::forbidden::Forbidden;
use std::sync::Arc;
use axum::{debug_handler, Json, Router};
use axum::extract::{State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use chrono::Utc;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use tokio_postgres::{Client, Transaction};
use utoipa::ToSchema;

use domain::entities::account::{Email, Password};
use domain::entities::account::create_account::CreateAccount;
use domain::entities::user::User;
use services::account_service::AccountService;
use services::account_service::mut_account_service::MutAccountService;
use crate::app_state::AppState;
use crate::controllers::{convert_error, convert_service_error};
use crate::extractors::headers::authorization::JWTAuthorization;
use crate::implementations::{get_account_repository, get_account_service, get_file_repository, get_image_repository, get_mut_account_repository, get_mut_account_service, get_mut_file_repository, get_mut_file_service, get_mut_image_repository, get_mut_image_service, get_mut_user_repository, get_mut_user_service, get_user_repository};
use crate::openapi::params::header::json_web_token::JsonWebTokenParam;
use crate::openapi::responses::bad_request::BadRequest;
use crate::openapi::responses::not_authorized::NotAuthorized;
use crate::openapi::responses::server_error::ServerError;

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

#[derive(ToSchema)]
pub struct AccountMultiPart {
  account: CreateAccount,
  profile_picture: Option<Vec<u8>>,
}
#[utoipa::path(post, path = "/register",
  responses(
    (status = 201, description = "Returned JWT and user. Valid for a week", body = LoginReturnData), ServerError, BadRequest
  ),
  request_body(content_type = ["multipart/form-data"], content = AccountMultiPart),
  tag = "Accounts"
)]
#[debug_handler]
async fn register(State(app_state): State<AppState>, MultiPartRequest(account): MultiPartRequest<CreateAccount>) -> Result<(StatusCode, Json<LoginReturnData>), (StatusCode, String)> {
  let mut connection = app_state.pool.get().await.map_err(convert_error)?;
  let transaction = connection.transaction().await.map_err(convert_error)?;

  let account = {
    let service = get_mut_service(&transaction, &app_state.display_path, &app_state.content_path);
    service.create(account).await.map_err(convert_service_error)?
  };
  transaction.commit().await.map_err(convert_error)?;

  let user_id = account.user.id;

  let in_a_week = (Utc::now().timestamp() + 604800) as usize;
  let claim = create_claim("Register".to_string(), user_id, in_a_week);
  let token = create_token(claim, app_state.secret.as_bytes())?;
  let user = account.user;
  Ok((StatusCode::CREATED, Json(LoginReturnData { token, user })))
}

fn create_token(claim: Claim, secret: &[u8]) -> Result<String, (StatusCode, String)> {
  let key = EncodingKey::from_secret(secret);
  let header = Header::default();

  jsonwebtoken::encode(&header, &claim, &key)
      .map_err(convert_error)
}

fn create_claim(subject: String, user_id: u32, exp: usize) -> Claim {
  Claim {
    user_id,
    sub: subject,
    iss: "MyCollection".to_string(),
    exp,
    iat: Utc::now().timestamp() as usize,
  }
}

#[utoipa::path(post, path = "/login",
responses(
(status = 200, description = "Returned JWT. Valid for a week", body = LoginReturnData), ServerError, NotAuthorized
),
request_body = LoginData,
tag = "Accounts"
)]
async fn login(State(app_state): State<AppState>, Json(login_data): Json<LoginData>) -> Result<(StatusCode, Json<LoginReturnData>), (StatusCode, String)> {
  let pooled = app_state.pool.get().await.unwrap();

  let password = login_data.password;
  let email = login_data.email;

  let account = {
    let service = get_service(&pooled);
    service.login(&Email(email), &Password(password)).await.map_err(convert_service_error)?
  };
  let in_a_week = (Utc::now().timestamp() + 604800) as usize;
  let claim = create_claim("Login".to_string(), account.user.id, in_a_week);
  let token = create_token(claim, app_state.secret.as_bytes())?;
  let user = account.user;
  Ok((StatusCode::OK, Json(LoginReturnData { token, user })))
}

#[utoipa::path(get, path = "/refresh",
  responses(
    (status = 200, description = "Returned JWT. Valid for an hour", body = String), ServerError, Forbidden
  ),
  params(JsonWebTokenParam),
  tag = "Accounts"
)]
async fn refresh_token(auth: JWTAuthorization, State(app_state): State<AppState>) -> impl IntoResponse {
  let claim = jsonwebtoken::decode::<Claim>(&auth.token, &DecodingKey::from_secret(app_state.secret.as_bytes()), &Validation::default());
  let Ok(claim) = claim else {
    return Err((StatusCode::FORBIDDEN, "Invalid JWT".to_string()));
  };
  let in_one_hour = (Utc::now().timestamp() as usize) + 3600;
  let claim = create_claim("Refresh".to_string(), claim.claims.user_id, in_one_hour);
  let token = create_token(claim, app_state.secret.as_bytes())?;
  Ok((StatusCode::OK, token))
}

#[derive(Debug, Serialize, Deserialize)]
struct Claim {
  user_id: u32,
  sub: String,
  exp: usize,
  iat: usize,
  iss: String,
}

fn get_mut_service<'a>(transaction: &'a Transaction, display_path: &'a str, path: &'a str) -> impl MutAccountService + 'a {
  let image_repository = Arc::new(get_image_repository(transaction.client()));
  let user_repository = Arc::new(get_user_repository(transaction.client(), image_repository.clone()));
  let account_repository = Arc::new(get_account_repository(transaction.client(), user_repository.clone()));
  let mut_account_repository = Arc::new(get_mut_account_repository(transaction, account_repository.clone(), user_repository.clone()));
  let mut_user_repository = Arc::new(get_mut_user_repository(transaction, user_repository, image_repository.clone()));
  let mut_file_repository = Arc::new(get_mut_file_repository());
  let file_repository = Arc::new(get_file_repository());
  let mut_image_repository = Arc::new(get_mut_image_repository(transaction, image_repository, mut_file_repository.clone(), file_repository));
  let mut_file_service = Arc::new(get_mut_file_service(mut_file_repository));
  //TODO: Make configurable
  let mut_image_service = Arc::new(get_mut_image_service(mut_image_repository, mut_file_service, display_path, path));
  let mut_user_service = Arc::new(get_mut_user_service(mut_user_repository, mut_image_service));
  let account_service = Arc::new(get_account_service(account_repository));
  let mut_account_service = get_mut_account_service(mut_account_repository, account_service, mut_user_service);
  mut_account_service
}

fn get_service(client: &Client) -> impl AccountService + '_ {
  let image_repository = Arc::new(get_image_repository(client));
  let user_repository = Arc::new(get_user_repository(client, image_repository));
  let account_repository = Arc::new(get_account_repository(client, user_repository));
  let account_service = get_account_service(account_repository);
  account_service
}
