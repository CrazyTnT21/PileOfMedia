use std::collections::HashMap;
use std::error::Error;
use std::sync::Arc;

use crate::convert_to_sql::to_i32;
use crate::schemas::db_user_book::DbUserBook;
use crate::select::expression::Expression;
use crate::select::Select;
use async_trait::async_trait;
use domain::entities::book::Book;
use domain::entities::user::user_book::UserBook;
use domain::enums::language::Language;
use domain::vec_tuple_to_map::vec_tuple_to_map;
use from_row::{RowColumns, Table};
use repositories::book_repository::BookRepository;
use repositories::user_repository::user_book_repository::UserBookRepository;
use tokio_postgres::Client;

pub struct DefaultUserBookRepository<'a> {
  client: &'a Client,
  book_repository: Arc<dyn BookRepository + 'a>,
}

impl<'a> DefaultUserBookRepository<'a> {
  pub fn new(client: &'a Client, book_repository: Arc<dyn BookRepository + 'a>) -> DefaultUserBookRepository<'a> {
    DefaultUserBookRepository {
      client,
      book_repository,
    }
  }
}
#[async_trait]
impl UserBookRepository for DefaultUserBookRepository<'_> {
  async fn get_by_book_id(
    &self,
    user_id: u32,
    book_id: u32,
    languages: &[Language],
  ) -> Result<Option<UserBook>, Box<dyn Error>> {
    let user_id = user_id as i32;
    let book_id = book_id as i32;
    let items = Select::new::<DbUserBook>()
      .columns_table::<DbUserBook>()
      .where_expression(Expression::value_equal(DbUserBook::TABLE_NAME, "fkbook", book_id))
      .where_expression(Expression::value_equal(DbUserBook::TABLE_NAME, "fkuser", user_id))
      .get_single_destruct(self.client)
      .await?;

    let Some(item) = items else {
      return Ok(None);
    };

    let book = self
      .book_repository
      .get_by_id(item.fk_book as u32, languages)
      .await?
      .unwrap();
    let result = item.to_entity(book);
    Ok(Some(result))
  }

  async fn get_by_book_ids(
    &self,
    user_id: u32,
    book_ids: &[u32],
    languages: &[Language],
  ) -> Result<Vec<UserBook>, Box<dyn Error>> {
    let user_id = user_id as i32;
    let book_ids = to_i32(book_ids);
    let items = Select::new::<DbUserBook>()
      .columns_table::<DbUserBook>()
      .where_expression(Expression::value_in((DbUserBook::TABLE_NAME, "fkbook"), &book_ids))
      .where_expression(Expression::value_equal(DbUserBook::TABLE_NAME, "fkuser", user_id))
      .query_destruct(self.client)
      .await?;
    let book_ids: Vec<u32> = items.iter().map(|x| x.fk_book as u32).collect();
    let books = self.book_repository.get_by_ids(&book_ids, languages).await?;

    let user_books = to_user_book(items, books);
    Ok(user_books)
  }

  async fn get_by_user_id(&self, user_id: u32, languages: &[Language]) -> Result<Vec<UserBook>, Box<dyn Error>> {
    let user_id = user_id as i32;

    let user_books = Select::new::<DbUserBook>()
      .where_expression(Expression::value_equal(DbUserBook::TABLE_NAME, "fkuser", user_id))
      .columns_table::<DbUserBook>()
      .query_destruct(self.client)
      .await?;

    let book_ids: Vec<u32> = user_books.iter().map(|x| x.fk_book as u32).collect();
    let books = self.book_repository.get_by_ids(&book_ids, languages).await?;

    let items = user_books
      .into_iter()
      .map(|x| {
        let book_id = x.fk_book;
        x.to_entity(books.iter().find(|y| y.id as i32 == book_id).unwrap().clone())
      })
      .collect();
    Ok(items)
  }

  async fn get_by_user_ids(
    &self,
    user_ids: &[u32],
    languages: &[Language],
  ) -> Result<HashMap<u32, Vec<UserBook>>, Box<dyn Error>> {
    let user_ids = to_i32(user_ids);
    let db_user_books = Select::new::<DbUserBook>()
      .columns_table::<DbUserBook>()
      .where_expression(Expression::value_in(column::<DbUserBook>("fkuser"), &user_ids))
      .query_destruct(self.client)
      .await?;

    let book_ids: Vec<u32> = db_user_books.iter().map(|x| x.fk_book as u32).collect();
    let books = self.book_repository.get_by_ids(&book_ids, languages).await?;

    let user_books: Vec<(u32, UserBook)> = db_user_books
      .into_iter()
      .map(|x| {
        let book_id = x.fk_book as u32;
        (
          x.fk_user as u32,
          x.to_entity(books.iter().find(|y| y.id == book_id).unwrap().clone()),
        )
      })
      .collect();
    let items = vec_tuple_to_map(user_books);
    Ok(items)
  }

  async fn filter_existing(&self, ids: HashMap<u32, Vec<u32>>) -> Result<HashMap<u32, Vec<u32>>, Box<dyn Error>> {
    let mut result = Vec::with_capacity(ids.values().len());
    for (user_id, book_ids) in ids {
      let user_id = user_id as i32;
      for book_id in book_ids {
        result.push((user_id, book_id as i32));
      }
    }
    let filtered = Select::new::<DbUserBook>()
      .column::<i32>(DbUserBook::TABLE_NAME, "fkuser")
      .column::<i32>(DbUserBook::TABLE_NAME, "fkbook")
      .where_expression(Expression::value_in(
        (column::<DbUserBook>("fkuser"), column::<DbUserBook>("fkbook")),
        &result,
      ))
      .query(self.client)
      .await?
      .into_iter()
      .map(|(x, y)| (x as u32, y as u32))
      .collect();
    let filtered = vec_tuple_to_map(filtered);

    Ok(filtered)
  }
}

fn to_user_book(items: Vec<DbUserBook>, mut books: Vec<Book>) -> Vec<UserBook> {
  items
    .into_iter()
    .map(|x| {
      let book_id = x.fk_book as u32;
      let result = x.to_entity(books.remove(books.iter().position(|y| y.id == book_id).unwrap()));
      result
    })
    .collect()
}
fn column<T: Table + RowColumns>(column: &str) -> (&'static str, &str) {
  if cfg!(debug_assertions) {
    assert!(T::COLUMNS.iter().map(|x| x.0).any(|x| x == column));
  }
  (T::TABLE_NAME, column)
}
