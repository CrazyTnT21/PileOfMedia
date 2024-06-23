use std::error::Error;

use async_trait::async_trait;
use tokio_postgres::Transaction;

use from_row::Table;
use repositories::book_repository::book_genre_repository::mut_book_genre_repository::MutBookGenreRepository;

use crate::convert_to_sql::{to_i32};
use crate::delete::Delete;
use crate::insert::Insert;
use crate::schemas::db_book_genre::DbBookGenre;
use crate::select::conditions::value_in::ValueIn;
use crate::select::expression::Expression;

pub struct DefaultMutBookGenreRepository<'a> {
  transaction: &'a Transaction<'a>,
}

impl<'a> DefaultMutBookGenreRepository<'a> {
  pub fn new(transaction: &'a Transaction<'a>) -> DefaultMutBookGenreRepository<'a> {
    DefaultMutBookGenreRepository { transaction }
  }
}

#[async_trait]
impl<'a> MutBookGenreRepository for DefaultMutBookGenreRepository<'a> {
  async fn add(&self, book_id: u32, genres: &[u32]) -> Result<(), Box<dyn Error>> {
    let book_id = book_id as i32;
    let genres = to_i32(genres);
    let mut insert = Insert::new::<DbBookGenre>(["fkbook", "fkgenre"]);
    genres.iter().for_each(|x| { insert.push_as_ref([&book_id, x]); });
    insert.execute_transaction(self.transaction).await?;
    Ok(())
  }

  async fn remove(&self, book_id: u32, genres: &[u32]) -> Result<(), Box<dyn Error>> {
    let book_id = book_id as i32;
    let genres = to_i32(genres);

    Delete::new::<DbBookGenre>(
      Expression::column_equal(DbBookGenre::TABLE_NAME, "fkbook", book_id)
        .and(Expression::new(ValueIn::new((DbBookGenre::TABLE_NAME, "fkgenre"), &genres)))
    )
      .execute_transaction(self.transaction)
      .await?;
    Ok(())
  }
}
