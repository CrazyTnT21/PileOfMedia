use bb8_postgres::bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::NoTls;

#[derive(Clone)]
pub struct AppState {
  pub pool: Pool<PostgresConnectionManager<NoTls>>,
  pub display_path: String,
  pub content_path: String,
  pub secret: String,
}
