use bb8_postgres::bb8::PooledConnection;
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::NoTls;

mod convert_to_sql;
pub mod delete;
pub mod enums;
mod fallback_unwrap;
pub mod insert;
pub mod macros;
pub mod repositories;
pub mod schemas;
pub mod select;

pub type Pooled<'a> = PooledConnection<'a, PostgresConnectionManager<NoTls>>;
