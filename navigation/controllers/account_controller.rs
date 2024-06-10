use std::error::Error;
use std::ops::Deref;
use std::sync::Arc;

use axum::{Json, Router};
use axum::extract::{FromRef, FromRequestParts};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use bb8_postgres::bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use chrono::Timelike;
use chrono::Utc;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use tokio_postgres::{Client, NoTls, Transaction};
use utoipa::ToSchema;

use domain::entities::account::{Email, Password};
use domain::entities::account::partial_create_account::PartialCreateAccount;
use repositories::account_repository::AccountRepository;
use repositories::account_repository::mut_account_repository::MutAccountRepository;
use repositories::file_repository::FileRepository;
use repositories::file_repository::mut_file_repository::MutFileRepository;
use repositories::image_repository::ImageRepository;
use repositories::image_repository::mut_image_repository::MutImageRepository;
use repositories::user_repository::mut_user_repository::MutUserRepository;
use repositories::user_repository::UserRepository;
use services::account_service::AccountService;
use services::account_service::mut_account_service::MutAccountService;
use services::image_service::mut_image_service::MutImageService;
use services::traits::service_error::ServiceError;
use services::user_service::mut_user_service::MutUserService;

use crate::controllers::convert_service_error;
use crate::database_connection::DatabaseConnection;
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

pub fn routes(pool: Pool<PostgresConnectionManager<NoTls>>) -> Router {
  Router::new()
    .route("/login", post(login))
    .route("/register", post(register))
    .route("/refresh", get(refresh_token))
    .with_state(pool)
}

#[utoipa::path(post, path = "/register",
responses(
(status = 200, description = "Returned JWT. Valid for a week", body = String), ServerError, BadRequest
),
request_body = PartialCreateAccount,
tag = "Accounts"
)]
async fn register(mut connection: DatabaseConnection, Json(account): Json<PartialCreateAccount>) -> Result<(StatusCode, String), (StatusCode, String)> {
  let transaction = connection.0
    .transaction()
    .await
    .map_err(map_postgres_error)?;

  let account = {
    let service = get_mut_service(&transaction);
    service.create(account).await.map_err(convert_service_error)?
  };
  transaction
    .commit()
    .await
    .map_err(map_postgres_error)?;

  let user_id = account.user.id;

  let in_a_week = (Utc::now().timestamp() + 604800) as usize;
  let claim = create_claim("Register".to_string(), user_id as u32, in_a_week);
  let token = create_token(claim)?;

  Ok((StatusCode::OK, token))
}

fn create_token(claim: Claim) -> Result<String, (StatusCode, String)> {
  //TODO use pem
  let key = EncodingKey::from_secret("TEST".as_ref());
  let header = Header::default();

  jsonwebtoken::encode(&header, &claim, &key)
    .map_err(|x| ServiceError::ServerError(Box::new(x)))
    .map_err(convert_service_error)
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
(status = 200, description = "Returned JWT. Valid for a week", body = String), ServerError, NotAuthorized
),
request_body = LoginData,
tag = "Accounts"
)]
async fn login(mut connection: DatabaseConnection, Json(login_data): Json<LoginData>) -> Result<String, (StatusCode, String)> {
  let pooled = connection.0;

  let password = login_data.password;
  let email = login_data.email;

  let account = {
    let service = get_service(&pooled);
    service.login(&Email(email), &Password(password)).await.map_err(convert_service_error)?
  };
  let in_a_week = (Utc::now().timestamp() + 604800) as usize;
  let claim = create_claim("Login".to_string(), account.user.id as u32, in_a_week);
  let token = create_token(claim)?;


  Ok(token)
}

#[utoipa::path(get, path = "/refresh",
  responses(
    (status = 200, description = "Returned JWT. Valid for an hour", body = String), ServerError, NotAuthorized
  ),
  params(JsonWebTokenParam),
  tag = "Accounts"
)]
async fn refresh_token(auth: JWTAuthorization) -> impl IntoResponse {
  let claim = jsonwebtoken::decode::<Claim>(&auth.token, &DecodingKey::from_secret("TEST".as_ref()), &Validation::default());
  let Ok(claim) = claim else {
    return Err((StatusCode::UNAUTHORIZED, "Invalid JWT".to_string()));
  };
  let in_one_hour = (Utc::now().timestamp() as usize) + 3600;
  let claim = create_claim("Refresh".to_string(), claim.claims.user_id, in_one_hour);
  let token = create_token(claim)?;
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

fn get_mut_service<'a>(transaction: &'a Transaction) -> impl MutAccountService + 'a {
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
  let mut_image_service = Arc::new(get_mut_image_service(mut_image_repository, mut_file_service, "http://localhost:3000/images/", ""));
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

fn map_postgres_error(error: tokio_postgres::Error) -> (StatusCode, String) {
  convert_service_error(ServiceError::ServerError(Box::new(error)))
}
