use bb8_postgres::bb8::PooledConnection;
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::NoTls;

pub mod macros;
pub mod select;
pub mod enums;
pub mod schemas;
pub mod repositories;
mod fallback_unwrap;

pub type Pooled<'a> = PooledConnection<'a, PostgresConnectionManager<NoTls>>;
