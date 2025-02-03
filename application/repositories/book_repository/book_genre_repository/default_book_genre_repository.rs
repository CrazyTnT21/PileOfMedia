use std::error::Error;
use std::sync::Arc;

use async_trait::async_trait;
use tokio_postgres::Client;

use domain::entities::genre::Genre;
use domain::enums::language::Language;
use from_row::Table;
use repositories::book_repository::book_genre_repository::BookGenreRepository;
use repositories::book_repository::BookRepository;
use repositories::genre_repository::GenreRepository;

use crate::convert_to_sql::to_i32;
use crate::schemas::db_book_genre::DbBookGenre;
use crate::select::conditions::value_equal::ValueEqual;
use crate::select::conditions::value_in::ValueIn;
use crate::select::expression::Expression;
use crate::select::Select;

pub struct DefaultBookGenreRepository<'a> {
  client: &'a Client,
  book_repository: Arc<dyn BookRepository + 'a>,
  genre_repository: Arc<dyn GenreRepository + 'a>,
}

impl<'a> DefaultBookGenreRepository<'a> {
  pub fn new(
    client: &'a Client,
    book_repository: Arc<dyn BookRepository + 'a>,
    genre_repository: Arc<dyn GenreRepository + 'a>,
  ) -> DefaultBookGenreRepository<'a> {
    DefaultBookGenreRepository {
      client,
      book_repository,
      genre_repository,
    }
  }
}

#[async_trait]
impl BookGenreRepository for DefaultBookGenreRepository<'_> {
  async fn get(&self, book_id: u32, languages: &[Language]) -> Result<Vec<Genre>, Box<dyn Error>> {
    let book_id = book_id as i32;

    let genre_ids: Vec<u32> = Select::new::<DbBookGenre>()
      .column::<i32>(DbBookGenre::TABLE_NAME, "fkgenre")
      .where_expression(Expression::new(ValueEqual::new(
        (DbBookGenre::TABLE_NAME, "fkbook"),
        book_id,
      )))
      .query(self.client)
      .await?
      .into_iter()
      .map(|x| x.0 as u32)
      .collect();
    let items = self.genre_repository.get_by_ids(&genre_ids, languages).await?;
    Ok(items)
  }

  async fn filter_existing(&self, book_id: u32, genres: &[u32]) -> Result<Vec<u32>, Box<dyn Error>> {
    let book_id = book_id as i32;
    let genres = to_i32(genres);

    let filtered = Select::new::<DbBookGenre>()
      .column::<i32>(DbBookGenre::TABLE_NAME, "fkgenre")
      .where_expression(Expression::new(ValueIn::new(
        (DbBookGenre::TABLE_NAME, "fkgenre"),
        &genres,
      )))
      .where_expression(Expression::value_equal(DbBookGenre::TABLE_NAME, "fkbook", book_id))
      .query(self.client)
      .await?
      .into_iter()
      .map(|x| x.0 as u32)
      .collect();
    Ok(filtered)
  }
}
