use std::error::Error;
use std::sync::Arc;

use async_trait::async_trait;
use tokio_postgres::Client;

use crate::convert_to_sql::to_i32;
use domain::entities::theme::Theme;
use domain::enums::language::Language;
use from_row::Table;
use repositories::book_repository::book_theme_repository::BookThemeRepository;
use repositories::book_repository::BookRepository;
use repositories::theme_repository::ThemeRepository;

use crate::schemas::db_book_theme::DbBookTheme;
use crate::select::conditions::value_equal::ValueEqual;
use crate::select::conditions::value_in::ValueIn;
use crate::select::expression::Expression;
use crate::select::Select;

pub struct DefaultBookThemeRepository<'a> {
  client: &'a Client,
  book_repository: Arc<dyn BookRepository + 'a>,
  theme_repository: Arc<dyn ThemeRepository + 'a>,
}

impl<'a> DefaultBookThemeRepository<'a> {
  pub fn new(
    client: &'a Client,
    book_repository: Arc<dyn BookRepository + 'a>,
    theme_repository: Arc<dyn ThemeRepository + 'a>,
  ) -> DefaultBookThemeRepository<'a> {
    DefaultBookThemeRepository {
      client,
      book_repository,
      theme_repository,
    }
  }
}

#[async_trait]
impl BookThemeRepository for DefaultBookThemeRepository<'_> {
  async fn get(&self, book_id: u32, languages: &[Language]) -> Result<Vec<Theme>, Box<dyn Error>> {
    let book_id = book_id as i32;

    let theme_ids: Vec<u32> = Select::new::<DbBookTheme>()
      .column::<i32>(DbBookTheme::TABLE_NAME, "fktheme")
      .where_expression(Expression::new(ValueEqual::new(
        (DbBookTheme::TABLE_NAME, "fkbook"),
        book_id,
      )))
      .query(self.client)
      .await?
      .into_iter()
      .map(|x| x.0 as u32)
      .collect();
    let items = self.theme_repository.get_by_ids(&theme_ids, languages).await?;
    Ok(items)
  }

  async fn filter_existing(&self, book_id: u32, themes: &[u32]) -> Result<Vec<u32>, Box<dyn Error>> {
    let book_id = book_id as i32;
    let themes = to_i32(themes);

    let filtered = Select::new::<DbBookTheme>()
      .column::<i32>(DbBookTheme::TABLE_NAME, "fktheme")
      .where_expression(Expression::new(ValueIn::new(
        (DbBookTheme::TABLE_NAME, "fktheme"),
        &themes,
      )))
      .where_expression(Expression::value_equal(DbBookTheme::TABLE_NAME, "fkbook", book_id))
      .query(self.client)
      .await?
      .into_iter()
      .map(|x| x.0 as u32)
      .collect();
    Ok(filtered)
  }
}
