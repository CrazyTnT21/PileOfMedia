use std::error::Error;
use async_trait::async_trait;
use bb8_postgres::bb8::PooledConnection;
use bb8_postgres::PostgresConnectionManager;
use chrono::NaiveDate;
use tokio_postgres::{NoTls};
use domain::entities::book::book::Book;
use domain::entities::franchise::franchise::Franchise;
use domain::entities::image::image::{Image, ImageExtension};
use domain::enums::language::Language;
use domain::pagination::Pagination;
use repositories::book_repository::BookRepository;


pub struct DefaultBookRepository {
  pool: PooledConnection<'static, PostgresConnectionManager<NoTls>>,
  default_language: Language,
}

impl DefaultBookRepository {
  pub fn new(pool: PooledConnection<'static, PostgresConnectionManager<NoTls>>, default_language: Language) -> DefaultBookRepository {
    DefaultBookRepository { pool, default_language }
  }
}

#[async_trait]
impl BookRepository for DefaultBookRepository {
  async fn get(&self, language: Language, pagination: Pagination) -> Result<Vec<Book>, Box<dyn Error>> {
    Ok(vec![fake_book()])
  }

  async fn get_by_id(&self, id: u32, language: Language) -> Result<Option<Book>, Box<dyn Error>> {
    Ok(Some(fake_book()))
  }

  async fn get_by_title(&self, title: &str, language: Language, pagination: Pagination) -> Result<Vec<Book>, Box<dyn Error>> {
    Ok(vec![fake_book(), fake_book()])
  }
}

fn fake_book() -> Book {
  Book {
    cover: Image {
      extension: Some(ImageExtension::JPG),
      height: 1002,
      id: 1,
      uri: String::from("https://Test"),
      width: 100,
    },
    id: 1,
    description: Some(String::from("Book description")),
    pages: Some(10),
    title: String::from("Book title"),
    words: Some(10),
    chapters: Some(1),
    published: NaiveDate::from_ymd_opt(2024, 2, 17),
    added: NaiveDate::MAX,
    members: 105,
    favorites: 2,
    rank: 6323,
    score: 6.25,
    popularity: 5545,
    franchise: Some(Franchise {
      id: 1,
      name: String::from("Book franchise"),
    }),
  }
}
