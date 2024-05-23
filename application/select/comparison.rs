use tokio_postgres::types::ToSql;

#[derive(Debug)]
pub enum Comparison<'a> {
  Equal(&'a (dyn ToSql + Sync)),
  NotEqual(&'a (dyn ToSql + Sync)),
  IsNull,
  IsNotNull,
  ILike(&'a String),
  In(&'a [&'a (dyn ToSql + Sync)]),
  Bigger(&'a (dyn ToSql + Sync)),
  BiggerEqual(&'a (dyn ToSql + Sync)),
  Less(&'a (dyn ToSql + Sync)),
  LessEqual(&'a (dyn ToSql + Sync)),
}
