use std::error::Error;

use async_trait::async_trait;
use tokio_postgres::Transaction;

use from_row::Table;
use repositories::book_repository::book_theme_repository::mut_book_theme_repository::MutBookThemeRepository;

use crate::convert_to_sql::to_i32;
use crate::delete::Delete;
use crate::insert::Insert;
use crate::schemas::db_book_theme::DbBookTheme;
use crate::select::conditions::value_in::ValueIn;
use crate::select::expression::Expression;

pub struct DefaultMutBookThemeRepository<'a> {
  transaction: &'a Transaction<'a>,
}

impl<'a> DefaultMutBookThemeRepository<'a> {
  pub const fn new(transaction: &'a Transaction<'a>) -> DefaultMutBookThemeRepository<'a> {
    DefaultMutBookThemeRepository { transaction }
  }
}

#[async_trait]
impl<'a> MutBookThemeRepository for DefaultMutBookThemeRepository<'a> {
  async fn add(&self, book_id: u32, themes: &[u32]) -> Result<(), Box<dyn Error>> {
    let book_id = book_id as i32;
    let themes = to_i32(themes);
    let mut insert = Insert::new::<DbBookTheme>(["fkbook", "fktheme"]);
    themes.iter().for_each(|x| {
      insert.values_ref([&book_id, x]);
    });
    insert.execute_transaction(self.transaction).await?;
    Ok(())
  }

  async fn remove(&self, book_id: u32, themes: &[u32]) -> Result<(), Box<dyn Error>> {
    let book_id = book_id as i32;
    let themes = to_i32(themes);

    Delete::new::<DbBookTheme>(
      Expression::column_equal(DbBookTheme::TABLE_NAME, "fkbook", book_id).and(Expression::new(ValueIn::new(
        (DbBookTheme::TABLE_NAME, "fktheme"),
        &themes,
      ))),
    )
    .execute_transaction(self.transaction)
    .await?;
    Ok(())
  }

  async fn remove_all(&self, book_ids: &[u32]) -> Result<(), Box<dyn Error>> {
    let book_ids = to_i32(book_ids);

    Delete::new::<DbBookTheme>(Expression::new(ValueIn::new(
      (DbBookTheme::TABLE_NAME, "fkbook"),
      &book_ids,
    )))
    .execute_transaction(self.transaction)
    .await?;
    Ok(())
  }
}
