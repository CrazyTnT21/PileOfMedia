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
      id: self.id as u32,
      title: book_translation.title,
      description: book_translation.description,
      chapters: self.chapters.map(|x| x as u16),
      pages: self.pages.map(|x| x as u16),
      words: self.words.map(|x| x as u32),
      published: self.published,
      cover,
      score: self.score,
      rank: self.rank as u32,
      popularity: self.popularity as u32,
      favorites: self.favorites as u32,
      members: self.members as u32,
      added: self.added,
      franchise,
    }
  }
}
