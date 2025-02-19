use crate::convert_to_sql::to_i32;
use crate::delete::Delete;
use crate::enums::db_user_status::DbUserStatus;
use crate::insert::Insert;
use crate::schemas::db_user_book::DbUserBook;
use crate::select::expression::Expression;
use async_trait::async_trait;
use domain::entities::user::create_user_book::CreateUserBook;
use domain::entities::user::user_book::UserBook;
use domain::enums::language::Language;
use from_row::Table;
use repositories::user_repository::user_book_repository::mut_user_book_repository::MutUserBookRepository;
use repositories::user_repository::user_book_repository::UserBookRepository;
use std::error::Error;
use std::sync::Arc;
use tokio_postgres::Transaction;

pub struct DefaultMutUserBookRepository<'a> {
  transaction: &'a Transaction<'a>,
  user_book_repository: Arc<dyn UserBookRepository + 'a>,
}

impl<'a> DefaultMutUserBookRepository<'a> {
  pub const fn new(
    transaction: &'a Transaction<'a>,
    user_book_repository: Arc<dyn UserBookRepository + 'a>,
  ) -> DefaultMutUserBookRepository<'a> {
    DefaultMutUserBookRepository {
      transaction,
      user_book_repository,
    }
  }
}

#[async_trait]
impl MutUserBookRepository for DefaultMutUserBookRepository<'_> {
  async fn add(&self, user_id: u32, book: CreateUserBook, languages: &[Language]) -> Result<UserBook, Box<dyn Error>> {
    let db_user_id = user_id as i32;
    let db_book_id = book.book_id as i32;
    let status = DbUserStatus::from(book.status);
    let chapters = book.chapters.map(|x| x as i16);
    let pages = book.pages.map(|x| x as i16);
    let score: Option<i16> = book.score.map(|x| x.to_u8().into());
    let insert = Insert::new::<DbUserBook>([
      "fkuser",
      "fkbook",
      "userstatus",
      "favorite",
      "score",
      "review",
      "start",
      "finished",
      "chapters",
      "pages",
    ])
    .values([
      &db_user_id,
      &db_book_id,
      &status,
      &book.favorite,
      &score,
      &book.review,
      &book.start,
      &book.finished,
      &chapters,
      &pages,
    ]);
    insert.execute_transaction(self.transaction).await?;
    let user_book = self
      .user_book_repository
      .get_by_book_id(user_id, book.book_id, languages)
      .await?;
    Ok(user_book.unwrap())
  }

  async fn remove(&self, user_id: u32, book_ids: &[u32]) -> Result<(), Box<dyn Error>> {
    let user_id = user_id as i32;
    let book_ids = to_i32(book_ids);

    Delete::new::<DbUserBook>(
      Expression::value_equal(DbUserBook::TABLE_NAME, "fkuser", user_id)
        .and(Expression::value_in((DbUserBook::TABLE_NAME, "fkbook"), &book_ids)),
    )
    .execute_transaction(self.transaction)
    .await?;
    Ok(())
  }

  async fn remove_all(&self, user_ids: &[u32]) -> Result<(), Box<dyn Error>> {
    let user_ids = to_i32(user_ids);

    Delete::new::<DbUserBook>(Expression::value_in((DbUserBook::TABLE_NAME, "fkuser"), &user_ids))
      .execute_transaction(self.transaction)
      .await?;
    Ok(())
  }
}
