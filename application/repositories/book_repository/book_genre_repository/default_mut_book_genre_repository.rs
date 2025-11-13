use std::error::Error;

use async_trait::async_trait;
use tokio_postgres::Transaction;

use from_row::Table;
use repositories::book_repository::book_genre_repository::mut_book_genre_repository::MutBookGenreRepository;

use crate::convert_to_sql::to_i32;
use crate::delete::Delete;
use crate::insert::Insert;
use crate::schemas::db_book_genre::DbBookGenre;
use crate::select::conditions::value_in::ValueIn;
use crate::select::expression::Expression;

pub struct DefaultMutBookGenreRepository<'a> {
  transaction: &'a Transaction<'a>,
}

impl<'a> DefaultMutBookGenreRepository<'a> {
  pub const fn new(transaction: &'a Transaction<'a>) -> DefaultMutBookGenreRepository<'a> {
    DefaultMutBookGenreRepository { transaction }
  }
}

#[async_trait]
impl MutBookGenreRepository for DefaultMutBookGenreRepository<'_> {
  async fn add(&self, book_id: u32, genres: &[u32]) -> Result<(), Box<dyn Error>> {
    let book_id = book_id as i32;
    let genres = to_i32(genres);
    let mut insert = Insert::new::<DbBookGenre>(["book_id", "genre_id"]);
    genres.iter().for_each(|x| {
      insert.values_ref([&book_id, x]);
    });
    insert.execute_transaction(self.transaction).await?;
    Ok(())
  }

  async fn remove(&self, book_id: u32, genres: &[u32]) -> Result<(), Box<dyn Error>> {
    let book_id = book_id as i32;
    let genres = to_i32(genres);

    Delete::new::<DbBookGenre>(Expression::value_equal(DbBookGenre::TABLE_NAME, "book_id", book_id).and(
      Expression::new(ValueIn::new((DbBookGenre::TABLE_NAME, "genre_id"), &genres)),
    ))
    .execute_transaction(self.transaction)
    .await?;
    Ok(())
  }
  async fn remove_all(&self, book_ids: &[u32]) -> Result<(), Box<dyn Error>> {
    let book_ids = to_i32(book_ids);

    Delete::new::<DbBookGenre>(Expression::new(ValueIn::new(
      (DbBookGenre::TABLE_NAME, "book_id"),
      &book_ids,
    )))
    .execute_transaction(self.transaction)
    .await?;
    Ok(())
  }
}
