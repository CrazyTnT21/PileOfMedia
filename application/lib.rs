use bb8_postgres::PostgresConnectionManager;
use bb8_postgres::bb8::PooledConnection;
use tokio_postgres::NoTls;

mod convert_to_sql;
pub mod delete;
pub mod enums;
pub mod insert;
pub mod macros;
pub mod repositories;
pub mod schemas;
pub mod select;

pub type Pooled<'a> = PooledConnection<'a, PostgresConnectionManager<NoTls>>;
