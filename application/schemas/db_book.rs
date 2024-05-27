use chrono::NaiveDate;
use tokio_postgres::Row;

use domain::entities::book::Book;
use domain::entities::franchise::Franchise;
use domain::entities::image::Image;
use from_row::FromRow;

use crate::schemas::db_book_translation::DbBookTranslation;

#[derive(FromRow, Debug)]
#[rename = "book"]
pub struct DbBook {
  pub id: i32,
  pub chapters: Option<i16>,
  pub pages: Option<i16>,
  pub words: Option<i32>,
  pub published: Option<NaiveDate>,
  pub score: f32,
  pub added: NaiveDate,
  pub rank: i32,
  pub popularity: i32,
  pub favorites: i32,
  pub members: i32,
  #[rename = "fkfranchise"]
  pub fk_franchise: Option<i32>,
}

impl DbBook {
  pub fn to_entity(self, book_translation: DbBookTranslation, cover: Image, franchise: Option<Franchise>) -> Book {
    Book {
      id: self.id,
      title: book_translation.title,
      description: book_translation.description,
      chapters: self.chapters,
      pages: self.pages,
      words: self.words,
      published: self.published,
      cover,
      score: self.score,
      rank: self.rank,
      popularity: self.popularity,
      favorites: self.favorites,
      members: self.members,
      added: self.added,
      franchise,
    }
  }
}
