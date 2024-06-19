use std::error::Error;

use async_trait::async_trait;
use tokio_postgres::Transaction;

use from_row::Table;
use repositories::book_repository::book_theme_repository::mut_book_theme_repository::MutBookThemeRepository;

use crate::convert_to_sql::{convert_to_sql, to_i32};
use crate::delete::Delete;
use crate::insert::Insert;
use crate::schemas::db_book_theme::DbBookTheme;
use crate::select::comparison::Comparison::In;
use crate::select::condition::Condition::Value;
use crate::select::expression::Expression;

pub struct DefaultMutBookThemeRepository<'a> {
  transaction: &'a Transaction<'a>,
}

impl<'a> DefaultMutBookThemeRepository<'a> {
  pub fn new(transaction: &'a Transaction<'a>) -> DefaultMutBookThemeRepository<'a> {
    DefaultMutBookThemeRepository { transaction }
  }
}

#[async_trait]
impl<'a> MutBookThemeRepository for DefaultMutBookThemeRepository<'a> {
  async fn add(&self, book_id: u32, themes: &[u32]) -> Result<(), Box<dyn Error>> {
    let book_id = book_id as i32;
    let themes = to_i32(themes);
    let mut insert = Insert::new::<DbBookTheme>(["fkbook", "fktheme"]);
    themes.iter().for_each(|x| { insert.push_as_ref([&book_id, x]); });
    insert.execute_transaction(self.transaction).await?;
    Ok(())
  }

  async fn remove(&self, book_id: u32, themes: &[u32]) -> Result<(), Box<dyn Error>> {
    let book_id = book_id as i32;
    let themes = to_i32(themes);
    let themes = convert_to_sql(&themes);
    Delete::new::<DbBookTheme>(
      Expression::column_equal(DbBookTheme::TABLE_NAME, "fkbook", &book_id)
        .and(Expression::new(Value((DbBookTheme::TABLE_NAME, "fktheme"), In(&themes))))
    )
      .execute_transaction(self.transaction)
      .await?;
    Ok(())
  }
}
