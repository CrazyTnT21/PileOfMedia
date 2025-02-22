use std::collections::HashMap;
use std::error::Error;
use std::sync::Arc;

use async_trait::async_trait;
use tokio_postgres::Client;

use domain::entities::theme::Theme;
use domain::enums::language::Language;
use domain::vec_tuple_to_map::vec_tuple_to_map;
use from_row::Table;
use repositories::book_repository::book_theme_repository::BookThemeRepository;
use repositories::theme_repository::ThemeRepository;

use crate::convert_to_sql::to_i32;
use crate::schemas::db_book_theme::DbBookTheme;
use crate::select::Select;
use crate::select::conditions::value_equal::ValueEqual;
use crate::select::conditions::value_in::ValueIn;
use crate::select::expression::Expression;

pub struct DefaultBookThemeRepository<'a> {
  client: &'a Client,
  theme_repository: Arc<dyn ThemeRepository + 'a>,
}

impl<'a> DefaultBookThemeRepository<'a> {
  pub fn new(client: &'a Client, theme_repository: Arc<dyn ThemeRepository + 'a>) -> DefaultBookThemeRepository<'a> {
    DefaultBookThemeRepository {
      client,
      theme_repository,
    }
  }
}

#[async_trait]
impl BookThemeRepository for DefaultBookThemeRepository<'_> {
  async fn get_by_id(&self, book_id: u32, languages: &[Language]) -> Result<Vec<Theme>, Box<dyn Error>> {
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

  async fn get_by_ids(
    &self,
    book_ids: &[u32],
    languages: &[Language],
  ) -> Result<HashMap<u32, Vec<Theme>>, Box<dyn Error>> {
    let book_ids = to_i32(book_ids);

    let ids = Select::new::<DbBookTheme>()
      .column::<i32>(DbBookTheme::TABLE_NAME, "fkbook")
      .column::<i32>(DbBookTheme::TABLE_NAME, "fktheme")
      .where_expression(Expression::new(ValueIn::new(
        (DbBookTheme::TABLE_NAME, "fkbook"),
        &book_ids,
      )))
      .query(self.client)
      .await?;

    let theme_ids: Vec<u32> = ids.iter().map(|x| x.1 as u32).collect();
    let items = self.theme_repository.get_by_ids(&theme_ids, languages).await?;
    let result = vec_tuple_to_map(ids)
      .into_iter()
      .map(|(id, themes)| {
        (
          id as u32,
          themes
            .into_iter()
            .map(|x| items.iter().find(|y| y.id as i32 == x).unwrap().clone())
            .collect::<Vec<Theme>>(),
        )
      })
      .collect();
    Ok(result)
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
