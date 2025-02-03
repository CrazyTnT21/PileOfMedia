use async_trait::async_trait;
use domain::available_translations::AvailableTranslations;
use std::collections::HashMap;
use std::error::Error;
use std::hash::Hash;
use tokio_postgres::Client;

use domain::entities::theme::theme_translation::ThemeTranslation;
use domain::entities::theme::Theme;
use domain::enums::language::Language;
use domain::items_total::ItemsTotal;
use domain::pagination::Pagination;
use from_row::{FromRow, Table};
use repositories::theme_repository::ThemeRepository;

use crate::convert_to_sql::to_i32;
use crate::enums::db_language::DbLanguage;
use crate::schemas::db_theme::DbTheme;
use crate::schemas::db_theme_translation::DbThemeTranslation;
use crate::select::combined_tuple::CombinedType;
use crate::select::conditions::value_equal::ValueEqual;
use crate::select::conditions::value_ilike::ValueILike;
use crate::select::conditions::value_in::ValueIn;
use crate::select::expression::Expression;
use crate::select::Select;

pub struct DefaultThemeRepository<'a> {
  client: &'a Client,
}

impl<'a> DefaultThemeRepository<'a> {
  pub const fn new(client: &'a Client) -> DefaultThemeRepository<'a> {
    DefaultThemeRepository { client }
  }
}

#[async_trait]
impl ThemeRepository for DefaultThemeRepository<'_> {
  async fn get(&self, languages: &[Language], pagination: Pagination) -> Result<ItemsTotal<Theme>, Box<dyn Error>> {
    let db_languages: Vec<DbLanguage> = languages.iter().map(|x| (*x).into()).collect();
    let total = Select::new::<DbTheme>()
      .transform(inner_join_translation)
      .query_count(self.client)
      .await? as usize;

    let themes = Select::new::<DbTheme>()
      .distinct_on(DbTheme::TABLE_NAME, "id")
      .columns_table::<DbTheme>()
      .transform(inner_join_translation)
      .pagination(pagination)
      .query_destruct(self.client)
      .await?;
    let theme_ids: Vec<i32> = themes.iter().map(|x| x.id).collect();

    let translations: Vec<(Language, i32, ThemeTranslation)> = theme_translation_select(&theme_ids, &db_languages)
      .query(self.client)
      .await?
      .into_iter()
      .map(|x| (x.0.language.into(), x.0.fk_translation, x.0.to_entity()))
      .collect();
    let translations = map_translation(&themes, translations);

    let available = self.available_languages(&theme_ids).await?;
    let themes = to_entities(themes, available, translations);
    Ok(ItemsTotal { items: themes, total })
  }

  async fn get_by_id(&self, id: u32, languages: &[Language]) -> Result<Option<Theme>, Box<dyn Error>> {
    let id = id as i32;
    let db_languages: Vec<DbLanguage> = languages.iter().map(|x| (*x).into()).collect();

    let themes = Select::new::<DbTheme>()
      .columns_table::<DbTheme>()
      .distinct_on(DbTheme::TABLE_NAME, "id")
      .inner_join::<DbThemeTranslation>(
        None,
        Expression::new(ValueEqual::new((DbTheme::TABLE_NAME, "id"), id)).and(theme_id_equal_fk_translation()),
      )
      .get_single(self.client)
      .await?;
    let Some(item) = themes else {
      return Ok(None);
    };
    let translations: Vec<(Language, ThemeTranslation)> = Select::new::<DbThemeTranslation>()
      .columns::<DbThemeTranslation>(DbThemeTranslation::TABLE_NAME)
      .where_expression(
        Expression::value_equal(DbThemeTranslation::TABLE_NAME, "fktranslation", item.0.id)
          .and(in_languages(&db_languages)),
      )
      .query(self.client)
      .await?
      .into_iter()
      .map(|x| (x.0.language.into(), x.0.to_entity()))
      .collect();
    let mut available = self.available_languages(&[id]).await?;
    let item = item.0.to_entity(AvailableTranslations {
      available_languages: available.remove(&id).unwrap(),
      translations: HashMap::from_iter(translations),
    });
    Ok(Some(item))
  }

  async fn get_by_ids(&self, ids: &[u32], languages: &[Language]) -> Result<Vec<Theme>, Box<dyn Error>> {
    let ids = to_i32(ids);
    let db_languages: Vec<DbLanguage> = languages.iter().map(|x| (*x).into()).collect();

    let themes = Select::new::<DbTheme>()
      .columns_table::<DbTheme>()
      .distinct_on(DbTheme::TABLE_NAME, "id")
      .transform(inner_join_translation)
      .where_expression(Expression::new(ValueIn::new((DbTheme::TABLE_NAME, "id"), &ids)))
      .query_destruct(self.client)
      .await?;

    if themes.is_empty() {
      return Ok(vec![]);
    }
    let theme_ids: Vec<i32> = themes.iter().map(|x| x.id).collect();

    let translations: Vec<(Language, i32, ThemeTranslation)> = theme_translation_select(&theme_ids, &db_languages)
      .query(self.client)
      .await?
      .into_iter()
      .map(|x| (x.0.language.into(), x.0.fk_translation, x.0.to_entity()))
      .collect();

    let translations = map_translation(&themes, translations);
    let available = self.available_languages(&theme_ids).await?;
    let themes = to_entities(themes, available, translations);
    Ok(themes)
  }

  async fn get_by_name(
    &self,
    name: &str,
    languages: &[Language],
    pagination: Pagination,
  ) -> Result<ItemsTotal<Theme>, Box<dyn Error>> {
    let name = format!("%{name}%");

    let db_languages: Vec<DbLanguage> = languages.iter().map(|x| (*x).into()).collect();
    let total = Select::new::<DbTheme>()
      .transform(|x| inner_join_translation_on_name(x, &name, &db_languages))
      .query_count(self.client)
      .await? as usize;

    let themes = Select::new::<DbTheme>()
      .columns_table::<DbTheme>()
      .distinct_on(DbTheme::TABLE_NAME, "id")
      .transform(|x| inner_join_translation_on_name(x, &name, &db_languages))
      .pagination(pagination)
      .query_destruct(self.client)
      .await?;
    let theme_ids: Vec<i32> = themes.iter().map(|x| x.id).collect();

    let translations: Vec<(Language, i32, ThemeTranslation)> = theme_translation_select(&theme_ids, &db_languages)
      .where_expression(theme_translation_with_name(&name))
      .query(self.client)
      .await?
      .into_iter()
      .map(|x| (x.0.language.into(), x.0.fk_translation, x.0.to_entity()))
      .collect();

    let translations = map_translation(&themes, translations);
    let available = self.available_languages(&theme_ids).await?;
    let themes = to_entities(themes, available, translations);
    Ok(ItemsTotal { items: themes, total })
  }

  async fn filter_existing(&self, themes: &[u32]) -> Result<Vec<u32>, Box<dyn Error>> {
    let themes = to_i32(themes);

    let count = Select::new::<DbTheme>()
      .column::<i32>(DbTheme::TABLE_NAME, "id")
      .where_expression(Expression::new(ValueIn::new((DbTheme::TABLE_NAME, "id"), &themes)))
      .query(self.client)
      .await?
      .into_iter()
      .map(|x| x.0 as u32)
      .collect();
    Ok(count)
  }
}
impl DefaultThemeRepository<'_> {
  async fn available_languages(&self, ids: &[i32]) -> Result<HashMap<i32, Vec<Language>>, Box<dyn Error>> {
    let available_translations = Select::new::<DbThemeTranslation>()
      .column::<i32>(DbThemeTranslation::TABLE_NAME, "fktranslation")
      .column::<DbLanguage>(DbThemeTranslation::TABLE_NAME, "language")
      .where_expression(Expression::new(ValueIn::new(
        (DbThemeTranslation::TABLE_NAME, "fktranslation"),
        ids,
      )))
      .query(self.client)
      .await?;
    let available_translations: Vec<(i32, Language)> = available_translations
      .into_iter()
      .map(|x| (x.0, Language::from(x.1)))
      .collect();
    let available = vec_tuple_to_map(available_translations);
    Ok(available)
  }
}

fn vec_tuple_to_map<K: Hash + Eq, V>(values: Vec<(K, V)>) -> HashMap<K, Vec<V>> {
  let mut map = HashMap::new();
  for (key, value) in values {
    match map.get_mut(&key) {
      None => {
        map.insert(key, vec![value]);
      }
      Some(v) => {
        v.push(value);
      }
    }
  }
  map
}
fn theme_translation_select<'a>(
  theme_ids: &'a [i32],
  db_languages: &'a [DbLanguage],
) -> Select<'a, (DbThemeTranslation,)> {
  Select::new::<DbThemeTranslation>()
    .columns::<DbThemeTranslation>(DbThemeTranslation::TABLE_NAME)
    .where_expression(Expression::new(ValueIn::new(
      (DbThemeTranslation::TABLE_NAME, "fktranslation"),
      theme_ids,
    )))
    .where_expression(in_languages(db_languages))
}
fn map_translation(
  themes: &[DbTheme],
  translations: Vec<(Language, i32, ThemeTranslation)>,
) -> HashMap<i32, Vec<(Language, ThemeTranslation)>> {
  let mut new_translations: HashMap<i32, Vec<(Language, ThemeTranslation)>> = HashMap::new();
  for theme in themes {
    new_translations.insert(theme.id, vec![]);
  }
  for (language, id, translation) in translations {
    let result = new_translations.get_mut(&id).unwrap();
    result.push((language, translation));
  }
  new_translations
}
fn theme_id_equal_fk_translation<'a>() -> Expression<'a> {
  Expression::column_equal(
    (DbTheme::TABLE_NAME, "id"),
    (DbThemeTranslation::TABLE_NAME, "fktranslation"),
  )
}
fn theme_translation_with_name(name: &String) -> Expression {
  Expression::new(ValueILike::new((DbThemeTranslation::TABLE_NAME, "name"), name))
}
fn inner_join_translation_on_name<'a, T: FromRow<DbType = T> + CombinedType>(
  select: Select<'a, T>,
  name: &'a String,
  db_languages: &'a [DbLanguage],
) -> Select<'a, T> {
  select.inner_join::<DbThemeTranslation>(
    None,
    Expression::value_i_like((DbThemeTranslation::TABLE_NAME, "name"), name)
      .and(in_languages(db_languages))
      .and(theme_id_equal_fk_translation()),
  )
}
fn to_entities(
  themes: Vec<DbTheme>,
  mut available: HashMap<i32, Vec<Language>>,
  mut translations: HashMap<i32, Vec<(Language, ThemeTranslation)>>,
) -> Vec<Theme> {
  themes
    .into_iter()
    .map(|theme| {
      let id = theme.id;
      theme.to_entity(AvailableTranslations {
        available_languages: available.remove(&id).unwrap(),
        translations: HashMap::from_iter(translations.remove(&id).unwrap()),
      })
    })
    .collect()
}
fn inner_join_translation<T: FromRow<DbType = T> + CombinedType>(select: Select<T>) -> Select<T> {
  select.inner_join::<DbThemeTranslation>(None, theme_id_equal_fk_translation())
}
fn in_languages(languages: &[DbLanguage]) -> Expression {
  Expression::new(ValueIn::new((DbThemeTranslation::TABLE_NAME, "language"), languages))
}
