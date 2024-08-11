use std::env;
use std::error::Error;
use axum::http::header::CONTENT_TYPE;

use axum::http::Method;
use bb8_postgres::bb8::{ManageConnection, Pool};
use bb8_postgres::PostgresConnectionManager;
use dotenvy::dotenv;
use tokio_postgres::NoTls;
use tower_http::cors::{Any, CorsLayer};

use crate::app_state::AppState;
use crate::controllers::route_controllers;

pub mod controllers;
mod extractors;
mod openapi;
mod implementations;
mod app_state;

pub async fn main() -> Result<(), Box<dyn Error>> {
  dotenv().ok();
  let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
  //let server_url = env::var("SERVER_URL").expect("SERVER_URL must be set");
  let content_path = env::var("CONTENT_PATH").expect("CONTENT_PATH must be set");
  let content_display_path = env::var("CONTENT_DISPLAY_PATH").expect("CONTENT_DISPLAY_PATH must be set");
  let secret = env::var("SECRET").expect("SECRET must be set");

  let pool = connection_pool(&database_url).await?;

  let cors = CorsLayer::new()
    .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
    .allow_headers([CONTENT_TYPE])
    .allow_origin(Any);

  let app_state = AppState {
    pool,
    display_path: content_display_path,
    content_path,
    secret,
  };

  let app = route_controllers(app_state)
    .layer(cors);

  let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
  println!("Server listening on port 3000!");
  axum::serve(listener, app).await?;
  Ok(())
}

async fn connection_pool(database_url: &str) -> Result<Pool<PostgresConnectionManager<NoTls>>, <PostgresConnectionManager<NoTls> as ManageConnection>::Error> {
  let manager =
    PostgresConnectionManager::new_from_stringlike(database_url, NoTls)?;
  Pool::builder().build(manager).await
}

