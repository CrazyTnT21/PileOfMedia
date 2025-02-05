use async_trait::async_trait;
use domain::available_translations::AvailableTranslations;
use std::collections::HashMap;
use std::error::Error;
use std::hash::Hash;
use tokio_postgres::Client;

use domain::entities::franchise::franchise_translation::FranchiseTranslation;
use domain::entities::franchise::Franchise;
use domain::enums::language::Language;
use domain::items_total::ItemsTotal;
use domain::pagination::Pagination;
use from_row::{FromRow, Table};
use repositories::franchise_repository::FranchiseRepository;

use crate::convert_to_sql::to_i32;
use crate::enums::db_language::DbLanguage;
use crate::schemas::db_franchise::DbFranchise;
use crate::schemas::db_franchise_translation::DbFranchiseTranslation;
use crate::select::combined_tuple::CombinedType;
use crate::select::conditions::value_equal::ValueEqual;
use crate::select::conditions::value_ilike::ValueILike;
use crate::select::conditions::value_in::ValueIn;
use crate::select::expression::Expression;
use crate::select::Select;

pub struct DefaultFranchiseRepository<'a> {
  client: &'a Client,
}

impl<'a> DefaultFranchiseRepository<'a> {
  pub const fn new(client: &'a Client) -> DefaultFranchiseRepository<'a> {
    DefaultFranchiseRepository { client }
  }
}

#[async_trait]
impl FranchiseRepository for DefaultFranchiseRepository<'_> {
  async fn get(&self, languages: &[Language], pagination: Pagination) -> Result<ItemsTotal<Franchise>, Box<dyn Error>> {
    let db_languages: Vec<DbLanguage> = languages.iter().map(|x| (*x).into()).collect();
    let total = Select::new::<DbFranchise>()
      .transform(inner_join_translation)
      .query_count(self.client)
      .await? as usize;

    let franchises = Select::new::<DbFranchise>()
      .distinct_on(DbFranchise::TABLE_NAME, "id")
      .columns_table::<DbFranchise>()
      .transform(inner_join_translation)
      .pagination(pagination)
      .query_destruct(self.client)
      .await?;
    let franchise_ids: Vec<i32> = franchises.iter().map(|x| x.id).collect();

    let mut translations: Vec<DbFranchiseTranslation> = franchise_translation_select(&franchise_ids, &db_languages)
      .query_destruct(self.client)
      .await?;

    let no_translations: Vec<i32> = no_translation_ids(&franchises, &translations);

    let mut extra_translations = Select::new::<DbFranchiseTranslation>()
      .distinct_on(DbFranchiseTranslation::TABLE_NAME, "fktranslation")
      .columns::<DbFranchiseTranslation>(DbFranchiseTranslation::TABLE_NAME)
      .where_expression(fk_translation_in_ids(&no_translations))
      .query_destruct(self.client)
      .await?;

    translations.append(&mut extra_translations);

    let translations: Vec<(Language, i32, FranchiseTranslation)> = translations
      .into_iter()
      .map(|x| (x.language.into(), x.fk_translation, x.to_entity()))
      .collect();

    let available = self.available_languages(&franchise_ids).await?;
    let translations = map_translation(&franchises, translations);

    let franchises = to_entities(franchises, available, translations);
    Ok(ItemsTotal {
      items: franchises,
      total,
    })
  }

  async fn get_by_id(&self, id: u32, languages: &[Language]) -> Result<Option<Franchise>, Box<dyn Error>> {
    let id = id as i32;
    let db_languages: Vec<DbLanguage> = languages.iter().map(|x| (*x).into()).collect();

    let franchises = Select::new::<DbFranchise>()
      .columns_table::<DbFranchise>()
      .distinct_on(DbFranchise::TABLE_NAME, "id")
      .inner_join::<DbFranchiseTranslation>(
        None,
        Expression::new(ValueEqual::new((DbFranchise::TABLE_NAME, "id"), id)).and(franchise_id_equal_fk_translation()),
      )
      .get_single_destruct(self.client)
      .await?;
    let Some(item) = franchises else {
      return Ok(None);
    };
    let mut translations = Select::new::<DbFranchiseTranslation>()
      .columns::<DbFranchiseTranslation>(DbFranchiseTranslation::TABLE_NAME)
      .where_expression(
        Expression::value_equal(DbFranchiseTranslation::TABLE_NAME, "fktranslation", item.id)
          .and(in_languages(&db_languages)),
      )
      .query_destruct(self.client)
      .await?;
    let items = [item];
    let no_translations: Vec<i32> = no_translation_ids(&items, &translations);
    let [item] = items;

    let mut extra_translations = Select::new::<DbFranchiseTranslation>()
      .distinct_on(DbFranchiseTranslation::TABLE_NAME, "fktranslation")
      .columns::<DbFranchiseTranslation>(DbFranchiseTranslation::TABLE_NAME)
      .where_expression(fk_translation_in_ids(&no_translations))
      .query_destruct(self.client)
      .await?;

    translations.append(&mut extra_translations);

    let translations: Vec<(Language, FranchiseTranslation)> = translations
      .into_iter()
      .map(|x| (x.language.into(), x.to_entity()))
      .collect();

    let mut available = self.available_languages(&[id]).await?;
    let item = item.to_entity(AvailableTranslations {
      available_languages: available.remove(&id).unwrap(),
      translations: HashMap::from_iter(translations),
    });
    Ok(Some(item))
  }

  async fn get_by_ids(&self, ids: &[u32], languages: &[Language]) -> Result<Vec<Franchise>, Box<dyn Error>> {
    let ids = to_i32(ids);
    let db_languages: Vec<DbLanguage> = languages.iter().map(|x| (*x).into()).collect();

    let franchises = Select::new::<DbFranchise>()
      .columns_table::<DbFranchise>()
      .distinct_on(DbFranchise::TABLE_NAME, "id")
      .transform(inner_join_translation)
      .where_expression(id_in_ids(&ids))
      .query_destruct(self.client)
      .await?;

    if franchises.is_empty() {
      return Ok(vec![]);
    }
    let franchise_ids: Vec<i32> = franchises.iter().map(|x| x.id).collect();

    let mut translations = franchise_translation_select(&franchise_ids, &db_languages)
      .query_destruct(self.client)
      .await?;

    let no_translations: Vec<i32> = no_translation_ids(&franchises, &translations);

    let mut extra_translations = Select::new::<DbFranchiseTranslation>()
      .distinct_on(DbFranchiseTranslation::TABLE_NAME, "fktranslation")
      .columns::<DbFranchiseTranslation>(DbFranchiseTranslation::TABLE_NAME)
      .where_expression(fk_translation_in_ids(&no_translations))
      .query_destruct(self.client)
      .await?;

    translations.append(&mut extra_translations);

    let translations: Vec<(Language, i32, FranchiseTranslation)> = translations
      .into_iter()
      .map(|x| (x.language.into(), x.fk_translation, x.to_entity()))
      .collect();

    let translations = map_translation(&franchises, translations);
    let available = self.available_languages(&franchise_ids).await?;
    let franchises = to_entities(franchises, available, translations);
    Ok(franchises)
  }

  async fn get_by_name(
    &self,
    name: &str,
    languages: &[Language],
    pagination: Pagination,
  ) -> Result<ItemsTotal<Franchise>, Box<dyn Error>> {
    let name = format!("%{name}%");

    let db_languages: Vec<DbLanguage> = languages.iter().map(|x| (*x).into()).collect();
    let total = Select::new::<DbFranchise>()
      .transform(|x| inner_join_translation_on_name(x, &name, &db_languages))
      .query_count(self.client)
      .await? as usize;

    let franchises = Select::new::<DbFranchise>()
      .columns_table::<DbFranchise>()
      .distinct_on(DbFranchise::TABLE_NAME, "id")
      .transform(|x| inner_join_translation_on_name(x, &name, &db_languages))
      .pagination(pagination)
      .query_destruct(self.client)
      .await?;
    let franchise_ids: Vec<i32> = franchises.iter().map(|x| x.id).collect();

    let mut translations = franchise_translation_select(&franchise_ids, &db_languages)
      .where_expression(franchise_translation_with_name(&name))
      .query_destruct(self.client)
      .await?;

    let no_translations: Vec<i32> = no_translation_ids(&franchises, &translations);

    let mut extra_translations = Select::new::<DbFranchiseTranslation>()
      .distinct_on(DbFranchiseTranslation::TABLE_NAME, "fktranslation")
      .columns::<DbFranchiseTranslation>(DbFranchiseTranslation::TABLE_NAME)
      .where_expression(fk_translation_in_ids(&no_translations))
      .query_destruct(self.client)
      .await?;

    translations.append(&mut extra_translations);

    let translations: Vec<(Language, i32, FranchiseTranslation)> = translations
      .into_iter()
      .map(|x| (x.language.into(), x.fk_translation, x.to_entity()))
      .collect();

    let translations = map_translation(&franchises, translations);
    let available = self.available_languages(&franchise_ids).await?;
    let franchises = to_entities(franchises, available, translations);
    Ok(ItemsTotal {
      items: franchises,
      total,
    })
  }

  async fn filter_existing(&self, franchises: &[u32]) -> Result<Vec<u32>, Box<dyn Error>> {
    let franchises = to_i32(franchises);

    let count = Select::new::<DbFranchise>()
      .column::<i32>(DbFranchise::TABLE_NAME, "id")
      .where_expression(id_in_ids(&franchises))
      .query(self.client)
      .await?
      .into_iter()
      .map(|x| x.0 as u32)
      .collect();
    Ok(count)
  }
}
impl DefaultFranchiseRepository<'_> {
  async fn available_languages(&self, ids: &[i32]) -> Result<HashMap<i32, Vec<Language>>, Box<dyn Error>> {
    let available_translations = Select::new::<DbFranchiseTranslation>()
      .column::<i32>(DbFranchiseTranslation::TABLE_NAME, "fktranslation")
      .column::<DbLanguage>(DbFranchiseTranslation::TABLE_NAME, "language")
      .where_expression(Expression::new(ValueIn::new(
        (DbFranchiseTranslation::TABLE_NAME, "fktranslation"),
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
fn franchise_translation_select<'a>(
  franchise_ids: &'a [i32],
  db_languages: &'a [DbLanguage],
) -> Select<'a, (DbFranchiseTranslation,)> {
  Select::new::<DbFranchiseTranslation>()
    .columns::<DbFranchiseTranslation>(DbFranchiseTranslation::TABLE_NAME)
    .where_expression(fk_translation_in_ids(franchise_ids))
    .where_expression(in_languages(db_languages))
}
fn map_translation(
  franchises: &[DbFranchise],
  translations: Vec<(Language, i32, FranchiseTranslation)>,
) -> HashMap<i32, Vec<(Language, FranchiseTranslation)>> {
  let mut new_translations: HashMap<i32, Vec<(Language, FranchiseTranslation)>> = HashMap::new();
  for franchise in franchises {
    new_translations.insert(franchise.id, vec![]);
  }
  for (language, id, translation) in translations {
    let result = new_translations.get_mut(&id).unwrap();
    result.push((language, translation));
  }
  new_translations
}
fn franchise_id_equal_fk_translation<'a>() -> Expression<'a> {
  Expression::column_equal(
    (DbFranchise::TABLE_NAME, "id"),
    (DbFranchiseTranslation::TABLE_NAME, "fktranslation"),
  )
}
fn franchise_translation_with_name(name: &String) -> Expression {
  Expression::new(ValueILike::new((DbFranchiseTranslation::TABLE_NAME, "name"), name))
}
fn inner_join_translation_on_name<'a, T: FromRow<DbType = T> + CombinedType>(
  select: Select<'a, T>,
  name: &'a String,
  db_languages: &'a [DbLanguage],
) -> Select<'a, T> {
  select.inner_join::<DbFranchiseTranslation>(
    None,
    Expression::value_i_like((DbFranchiseTranslation::TABLE_NAME, "name"), name)
      .and(in_languages(db_languages))
      .and(franchise_id_equal_fk_translation()),
  )
}
fn to_entities(
  franchises: Vec<DbFranchise>,
  mut available: HashMap<i32, Vec<Language>>,
  mut translations: HashMap<i32, Vec<(Language, FranchiseTranslation)>>,
) -> Vec<Franchise> {
  franchises
    .into_iter()
    .map(|franchise| {
      let id = franchise.id;
      franchise.to_entity(AvailableTranslations {
        available_languages: available.remove(&id).unwrap(),
        translations: HashMap::from_iter(translations.remove(&id).unwrap()),
      })
    })
    .collect()
}
fn inner_join_translation<T: FromRow<DbType = T> + CombinedType>(select: Select<T>) -> Select<T> {
  select.inner_join::<DbFranchiseTranslation>(None, franchise_id_equal_fk_translation())
}
fn in_languages(languages: &[DbLanguage]) -> Expression {
  Expression::new(ValueIn::new(
    (DbFranchiseTranslation::TABLE_NAME, "language"),
    languages,
  ))
}
fn fk_translation_in_ids(ids: &[i32]) -> Expression {
  Expression::new(ValueIn::new((DbFranchiseTranslation::TABLE_NAME, "fktranslation"), ids))
}
fn id_in_ids(ids: &[i32]) -> Expression {
  Expression::new(ValueIn::new((DbFranchise::TABLE_NAME, "id"), ids))
}
fn no_translation_ids(franchise_ids: &[DbFranchise], translations: &[DbFranchiseTranslation]) -> Vec<i32> {
  franchise_ids
    .iter()
    .filter_map(|x| {
      translations
        .iter()
        .find(|y| y.fk_translation == x.id)
        .map_or(Some(x.id), |_| None)
    })
    .collect()
}
