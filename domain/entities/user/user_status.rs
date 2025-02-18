use std::error::Error;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum UserStatus {
  NotStarted,
  Ongoing,
  Finished,
  Paused,
}

impl Display for UserStatus {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}",
      match self {
        UserStatus::NotStarted => "Not started",
        UserStatus::Ongoing => "Ongoing",
        UserStatus::Finished => "Finished",
        UserStatus::Paused => "Paused",
      }
    )
  }
}

#[derive(Debug)]
pub enum UserStatusError {
  UnknownUserStatus(String),
}

impl Display for UserStatusError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}",
      match self {
        UserStatusError::UnknownUserStatus(value) => format!("Unknown UserStatus, {value}"),
      }
    )
  }
}

impl Error for UserStatusError {}
impl FromStr for UserStatus {
  type Err = UserStatusError;

  //noinspection SpellCheckingInspection
  fn from_str(value: &str) -> Result<Self, Self::Err> {
    let result = match value.to_lowercase().as_str() {
      "notstarted" | "not_started" => UserStatus::NotStarted,
      "ongoing" => UserStatus::Ongoing,
      "finished" => UserStatus::Finished,
      "paused" => UserStatus::Paused,
      _ => Err(UserStatusError::UnknownUserStatus(value.to_string()))?,
    };
    Ok(result)
  }
}
