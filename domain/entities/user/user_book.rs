use crate::entities::book::Book;
use crate::entities::user::user_status::UserStatus;
use crate::score::Score;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct UserBook {
  pub book: Book,
  pub status: UserStatus,
  pub favorite: bool,
  pub score: Option<Score>,
  pub review: Option<String>,
  pub start: Option<chrono::NaiveDate>,
  pub finished: Option<chrono::NaiveDate>,
  pub chapters: Option<u16>,
  pub pages: Option<u16>,
}

pub type BookId = u32;
pub type UserId = u32;
