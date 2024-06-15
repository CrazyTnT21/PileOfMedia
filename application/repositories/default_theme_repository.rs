use std::error::Error;

use async_trait::async_trait;
use tokio_postgres::Client;

use domain::entities::theme::Theme;
use domain::enums::language::Language;
use domain::items_total::ItemsTotal;
use domain::pagination::Pagination;
use from_row::FromRow;
use repositories::theme_repository::ThemeRepository;

use crate::convert_to_sql::{convert_to_sql, to_i32};
use crate::enums::db_language::DbLanguage;
use crate::fallback_unwrap::fallback_unwrap;
use crate::schemas::db_theme::DbTheme;
use crate::schemas::db_theme_translation::DbThemeTranslation;
use crate::select::combined_tuple::CombinedType;
use crate::select::comparison::Comparison::{Equal, ILike, In};
use crate::select::condition::Condition::{Column, Value};
use crate::select::expression::Expression;
use crate::select::Select;

pub struct DefaultThemeRepository<'a> {
  client: &'a Client,
  default_language: DbLanguage,
}

impl<'a> DefaultThemeRepository<'a> {
  pub fn new(client: &'a Client, language: Language) -> DefaultThemeRepository<'a> {
    DefaultThemeRepository { client, default_language: language.into() }
  }
}

#[async_trait]
impl<'a> ThemeRepository for DefaultThemeRepository<'a> {
  async fn get(&self, language: Language, pagination: Pagination) -> Result<ItemsTotal<Theme>, Box<dyn Error>> {
    let language = DbLanguage::from(language);
    let select = theme_select_columns()
      .transform(|x| self.theme_joins(x, &language));

    let total = select.count(self.client).await? as usize;

    let themes = select
      .pagination(pagination)
      .query(self.client)
      .await?
      .into_iter()
      .map(to_entity)
      .collect();

    Ok(ItemsTotal { items: themes, total })
  }

  async fn get_by_id(&self, id: u32, language: Language) -> Result<Option<Theme>, Box<dyn Error>> {
    let id = id as i32;
    let language = DbLanguage::from(language);
    let theme = theme_select_columns()
      .transform(|x| self.theme_joins(x, &language))
      .where_expression(Expression::new(Value(("theme", "id"), Equal(&id))))
      .get_single(self.client)
      .await?;

    Ok(theme.map(to_entity))
  }

  async fn get_by_ids(&self, ids: &[u32], language: Language) -> Result<Vec<Theme>, Box<dyn Error>> {
    let language = DbLanguage::from(language);
    let ids = to_i32(ids);
    let ids = convert_to_sql(&ids);
    let themes = theme_select_columns()
      .transform(|x| self.theme_joins(x, &language))
      .where_expression(Expression::new(Value(("theme", "id"), In(&ids))))
      .query(self.client)
      .await?
      .into_iter()
      .map(to_entity)
      .collect();

    Ok(themes)
  }

  async fn get_by_name(&self, name: &str, language: Language, pagination: Pagination) -> Result<ItemsTotal<Theme>, Box<dyn Error>> {
    let language = DbLanguage::from(language);
    let name = format!("%{name}%");
    let select = theme_select_columns()
      .transform(|x| self.theme_joins(x, &language))
      .where_expression(Expression::new(Value(("theme_translation", "name"), ILike(&name)))
        .or(Expression::new(Value(("theme_translation_fallback", "name"), ILike(&name)))));

    let total = select.count(self.client).await? as usize;

    let themes = select
      .pagination(pagination)
      .query(self.client)
      .await?
      .into_iter()
      .map(to_entity)
      .collect();

    Ok(ItemsTotal { items: themes, total })
  }
}

impl<'a> DefaultThemeRepository<'a> {
  fn theme_joins<T: FromRow<DbType=T> + CombinedType>(&'a self, select: Select<'a, T>, language: &'a DbLanguage) -> Select<'a, T> {
    select
      .left_join::<DbThemeTranslation>(
        Some("theme_translation"),
        Expression::column_equal("theme_translation", "language", language)
          .and(Expression::new(Column(("theme_translation", "fktranslation"), ("theme", "id")))),
      )
      .left_join::<DbThemeTranslation>(
        Some("theme_translation_fallback"),
        Expression::column_equal("theme_translation_fallback", "language", &self.default_language)
          .and(Expression::new(Column(("theme_translation_fallback", "fktranslation"), ("theme", "id"))))
          .and(Expression::column_null("theme_translation", "fktranslation")),
      )
  }
}

fn to_entity(theme: (DbTheme, Option<DbThemeTranslation>, Option<DbThemeTranslation>)) -> Theme {
  theme.0.to_entity(fallback_unwrap(theme.1, theme.2))
}

fn theme_select_columns<'a>() -> Select<'a, ThemeColumns> {
  Select::new::<DbTheme>()
    .columns::<DbTheme>("theme")
    .columns::<Option<DbThemeTranslation>>("theme_translation")
    .columns::<Option<DbThemeTranslation>>("theme_translation_fallback")
}

type ThemeColumns = (DbTheme, Option<DbThemeTranslation>, Option<DbThemeTranslation>);
