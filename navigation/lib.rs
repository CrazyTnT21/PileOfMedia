use std::env;

use axum::http::Method;
use axum::Router;
use bb8_postgres::bb8::{ManageConnection, Pool};
use bb8_postgres::PostgresConnectionManager;
use dotenvy::dotenv;
use tokio_postgres::NoTls;
use tower_http::cors::{Any, CorsLayer};

use crate::controllers::route_controllers;

pub mod controllers;
pub mod headers;
mod database_connection;

pub async fn main() -> std::io::Result<()> {
  dotenv().ok();
  let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
  let pool = connection_pool(&database_url).await.unwrap();

  let cors = CorsLayer::new()
    .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
    .allow_origin(Any);

  let app = route_controllers(pool, Router::new())
    .layer(cors);

  let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
  axum::serve(listener, app).await
}

async fn connection_pool(database_url: &str) -> Result<Pool<PostgresConnectionManager<NoTls>>, <PostgresConnectionManager<NoTls> as ManageConnection>::Error> {
  let manager =
    PostgresConnectionManager::new_from_stringlike(&database_url, NoTls)?;
  Ok(Pool::builder().build(manager).await?)
}

