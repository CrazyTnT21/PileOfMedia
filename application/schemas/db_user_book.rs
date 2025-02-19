use crate::enums::db_user_status::DbUserStatus;
use domain::entities::book::Book;
use domain::entities::user::user_book::UserBook;
use from_row::FromRow;
use tokio_postgres::Row;

#[derive(FromRow, Clone, Debug)]
#[rename = "userbook"]
pub struct DbUserBook {
  #[rename = "fkuser"]
  pub fk_user: i32,
  #[rename = "fkbook"]
  pub fk_book: i32,
  #[rename = "userstatus"]
  pub status: DbUserStatus,
  pub favorite: bool,
  pub score: Option<i16>,
  pub review: Option<String>,
  pub start: Option<chrono::NaiveDate>,
  pub finished: Option<chrono::NaiveDate>,
  pub chapters: Option<i16>,
  pub pages: Option<i16>,
  pub added: chrono::NaiveDate,
}
impl DbUserBook {
  /// # Panics
  ///
  /// Will panic if the score is not between 1 and 10. This could only happen if the value was not validated when inserted.
  pub fn to_entity(self, book: Book) -> UserBook {
    UserBook {
      book,
      status: self.status.into(),
      favorite: self.favorite,
      score: self.score.map(|x| u8::try_from(x).unwrap().try_into().unwrap()),
      review: self.review,
      start: self.start,
      finished: self.finished,
      chapters: self.chapters.map(|x| x as u16),
      pages: self.pages.map(|x| x as u16),
    }
  }
}
