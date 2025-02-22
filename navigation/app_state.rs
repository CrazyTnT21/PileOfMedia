use bb8_postgres::PostgresConnectionManager;
use bb8_postgres::bb8::Pool;
use tokio_postgres::NoTls;

#[derive(Clone)]
pub struct AppState {
  pub pool: Pool<PostgresConnectionManager<NoTls>>,
  pub display_path: String,
  pub content_path: String,
  pub secret: String,
}
