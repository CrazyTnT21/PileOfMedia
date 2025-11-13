use from_row::FromRow;
use from_row::FromRowOption;
use std::error::Error;
use std::str::FromStr;

use serde::{Deserialize, Serialize};
use tokio_postgres::types::private::BytesMut;
use tokio_postgres::types::{FromSql, IsNull, Kind, ToSql, Type, to_sql_checked};

use domain::entities::user::user_status::UserStatus;
use from_row::from_row_impl;

use crate::{convert, enum_from_sql};

#[derive(Serialize, Deserialize, Copy, PartialEq, Eq, Clone, Debug)]
pub enum DbUserStatus {
  NotStarted,
  Ongoing,
  Finished,
  Paused,
}
from_row_impl!(DbUserStatus);
convert!(DbUserStatus, UserStatus, NotStarted, Ongoing, Finished, Paused);
enum_from_sql!(DbUserStatus, "user_status");

impl FromStr for DbUserStatus {
  type Err = <UserStatus as FromStr>::Err;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    UserStatus::from_str(s).map(DbUserStatus::from)
  }
}
impl ToSql for DbUserStatus {
  fn to_sql(&self, _ty: &Type, out: &mut BytesMut) -> Result<IsNull, Box<dyn Error + Sync + Send>>
  where
    Self: Sized,
  {
    out.extend_from_slice(&bytes(*self));
    Ok(IsNull::No)
  }

  fn accepts(ty: &Type) -> bool
  where
    Self: Sized,
  {
    <DbUserStatus as FromSql>::accepts(ty)
  }
  to_sql_checked!();
}
fn bytes(value: DbUserStatus) -> Vec<u8> {
  match value {
    DbUserStatus::NotStarted => "not_started",
    DbUserStatus::Ongoing => "ongoing",
    DbUserStatus::Finished => "finished",
    DbUserStatus::Paused => "paused",
  }
  .bytes()
  .collect()
}
