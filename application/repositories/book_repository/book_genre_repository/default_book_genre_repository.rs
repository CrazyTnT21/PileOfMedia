use std::collections::HashMap;
use std::error::Error;
use std::sync::Arc;

use async_trait::async_trait;
use tokio_postgres::Client;

use domain::entities::genre::Genre;
use domain::enums::language::Language;
use domain::vec_tuple_to_map::vec_tuple_to_map;
use from_row::Table;
use repositories::book_repository::book_genre_repository::BookGenreRepository;
use repositories::genre_repository::GenreRepository;

use crate::convert_to_sql::to_i32;
use crate::schemas::db_book_genre::DbBookGenre;
use crate::select::conditions::value_equal::ValueEqual;
use crate::select::conditions::value_in::ValueIn;
use crate::select::expression::Expression;
use crate::select::Select;

pub struct DefaultBookGenreRepository<'a> {
  client: &'a Client,
  genre_repository: Arc<dyn GenreRepository + 'a>,
}

impl<'a> DefaultBookGenreRepository<'a> {
  pub fn new(client: &'a Client, genre_repository: Arc<dyn GenreRepository + 'a>) -> DefaultBookGenreRepository<'a> {
    DefaultBookGenreRepository {
      client,
      genre_repository,
    }
  }
}

#[async_trait]
impl BookGenreRepository for DefaultBookGenreRepository<'_> {
  async fn get_by_id(&self, book_id: u32, languages: &[Language]) -> Result<Vec<Genre>, Box<dyn Error>> {
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

  async fn get_by_ids(
    &self,
    book_ids: &[u32],
    languages: &[Language],
  ) -> Result<HashMap<u32, Vec<Genre>>, Box<dyn Error>> {
    let book_ids = to_i32(book_ids);

    let ids = Select::new::<DbBookGenre>()
      .column::<i32>(DbBookGenre::TABLE_NAME, "fkbook")
      .column::<i32>(DbBookGenre::TABLE_NAME, "fkgenre")
      .where_expression(Expression::new(ValueIn::new(
        (DbBookGenre::TABLE_NAME, "fkbook"),
        &book_ids,
      )))
      .query(self.client)
      .await?;

    let genre_ids: Vec<u32> = ids.iter().map(|x| x.1 as u32).collect();
    let items = self.genre_repository.get_by_ids(&genre_ids, languages).await?;
    let result = vec_tuple_to_map(ids)
      .into_iter()
      .map(|(id, genres)| {
        (
          id as u32,
          genres
            .into_iter()
            .map(|x| items.iter().find(|y| y.id as i32 == x).unwrap().clone())
            .collect::<Vec<Genre>>(),
        )
      })
      .collect();
    Ok(result)
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
