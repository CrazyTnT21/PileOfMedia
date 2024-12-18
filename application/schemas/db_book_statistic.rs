use chrono::NaiveDate;
use domain::entities::book::book_statistic::BookStatistic;
use domain::entities::rating::Rating;
use from_row::FromRow;
use tokio_postgres::Row;

#[derive(FromRow, Debug)]
#[rename = "bookstatistic"]
pub struct DbBookStatistic {
  #[rename = "fkbook"]
  pub fk_book: i32,
  #[rename = "fkrating"]
  pub fk_rating: i32,
  pub added: NaiveDate,
  pub rank: i32,
  pub popularity: i32,
  pub favorites: i32,
  pub members: i32,
}
impl DbBookStatistic {
  pub const fn to_entity(self, rating: Rating) -> BookStatistic {
    BookStatistic {
      rating,
      rank: self.rank as u32,
      popularity: self.popularity as u32,
      favorites: self.favorites as u32,
      members: self.members as u32,
      added: self.added,
    }
  }
}
