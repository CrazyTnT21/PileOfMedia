use axum::http::Method;
use axum::http::header::{AUTHORIZATION, CONTENT_TYPE};
use axum::routing::get;
use bb8_postgres::PostgresConnectionManager;
use bb8_postgres::bb8::{ManageConnection, Pool};
use std::env;
use std::error::Error;
use tokio_postgres::NoTls;
use tower_http::cors::{Any, CorsLayer};

use crate::app_state::AppState;
use crate::controllers::{openapi_spec, route_controllers};

mod app_state;
pub mod controllers;
mod extractors;
mod implementations;
mod jwt;
mod openapi;

pub async fn main() -> Result<(), Box<dyn Error>> {
  load_local_env()?;
  let database_url = env::var("DATABASE_URL").map_err(|_| "DATABASE_URL must be set")?;
  //let server_url = env::var("SERVER_URL").expect("SERVER_URL must be set");
  let content_path = env::var("CONTENT_PATH").map_err(|_| "CONTENT_PATH must be set")?;
  let api_url = env::var("API_URL").map_err(|_| "API_URL must be set")?;
  let content_display_path = env::var("CONTENT_DISPLAY_PATH").map_err(|_| "CONTENT_DISPLAY_PATH must be set")?;
  let secret = env::var("SECRET").map_err(|_| "SECRET must be set")?;

  let pool = connection_pool(&database_url).await?;

  let cors = CorsLayer::new()
    .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
    .allow_headers([CONTENT_TYPE, AUTHORIZATION])
    .allow_origin(Any);

  let app_state = AppState {
    pool,
    display_path: content_display_path,
    content_path,
    secret,
  };
  let doc = openapi_spec(&api_url);
  let app = route_controllers(app_state).layer(cors);
  let app = app.route("/", get(|| async { axum::Json(doc) }));

  let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
  println!("Server listening on port 3000!");
  axum::serve(listener, app).await?;
  Ok(())
}

async fn connection_pool(
  database_url: &str,
) -> Result<Pool<PostgresConnectionManager<NoTls>>, <PostgresConnectionManager<NoTls> as ManageConnection>::Error> {
  let manager = PostgresConnectionManager::new_from_stringlike(database_url, NoTls)?;
  Pool::builder().build(manager).await
}

fn load_local_env() -> dotenvy::Result<()> {
  dotenvy::from_path_override(".local/.env").or_else(|e| e.not_found().ok_or_stable(e))
}
trait OkOr {
  fn ok_or_stable<E>(self, err: E) -> Result<(), E>;
}
impl OkOr for bool {
  #[inline]
  fn ok_or_stable<E>(self, err: E) -> Result<(), E> {
    if self { Ok(()) } else { Err(err) }
  }
}
