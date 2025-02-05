use async_trait::async_trait;
use domain::available_translations::AvailableTranslations;
use std::collections::HashMap;
use std::error::Error;
use std::hash::Hash;
use tokio_postgres::Client;

use domain::entities::role::role_translation::RoleTranslation;
use domain::entities::role::Role;
use domain::enums::language::Language;
use domain::items_total::ItemsTotal;
use domain::pagination::Pagination;
use from_row::{FromRow, Table};
use repositories::role_repository::RoleRepository;

use crate::convert_to_sql::to_i32;
use crate::enums::db_language::DbLanguage;
use crate::schemas::db_role::DbRole;
use crate::schemas::db_role_translation::DbRoleTranslation;
use crate::select::combined_tuple::CombinedType;
use crate::select::conditions::value_equal::ValueEqual;
use crate::select::conditions::value_ilike::ValueILike;
use crate::select::conditions::value_in::ValueIn;
use crate::select::expression::Expression;
use crate::select::Select;

pub struct DefaultRoleRepository<'a> {
  client: &'a Client,
}

impl<'a> DefaultRoleRepository<'a> {
  pub const fn new(client: &'a Client) -> DefaultRoleRepository<'a> {
    DefaultRoleRepository { client }
  }
}

#[async_trait]
impl RoleRepository for DefaultRoleRepository<'_> {
  async fn get(&self, languages: &[Language], pagination: Pagination) -> Result<ItemsTotal<Role>, Box<dyn Error>> {
    let db_languages: Vec<DbLanguage> = languages.iter().map(|x| (*x).into()).collect();
    let total = Select::new::<DbRole>()
      .transform(inner_join_translation)
      .query_count(self.client)
      .await? as usize;

    let roles = Select::new::<DbRole>()
      .distinct_on(DbRole::TABLE_NAME, "id")
      .columns_table::<DbRole>()
      .transform(inner_join_translation)
      .pagination(pagination)
      .query_destruct(self.client)
      .await?;
    let role_ids: Vec<i32> = roles.iter().map(|x| x.id).collect();

    let mut translations: Vec<DbRoleTranslation> = role_translation_select(&role_ids, &db_languages)
      .query_destruct(self.client)
      .await?;

    let no_translations: Vec<i32> = no_translation_ids(&roles, &translations);

    let mut extra_translations = Select::new::<DbRoleTranslation>()
      .distinct_on(DbRoleTranslation::TABLE_NAME, "fktranslation")
      .columns::<DbRoleTranslation>(DbRoleTranslation::TABLE_NAME)
      .where_expression(fk_translation_in_ids(&no_translations))
      .query_destruct(self.client)
      .await?;

    translations.append(&mut extra_translations);

    let translations: Vec<(Language, i32, RoleTranslation)> = translations
      .into_iter()
      .map(|x| (x.language.into(), x.fk_translation, x.to_entity()))
      .collect();

    let available = self.available_languages(&role_ids).await?;
    let translations = map_translation(&roles, translations);

    let roles = to_entities(roles, available, translations);
    Ok(ItemsTotal { items: roles, total })
  }

  async fn get_by_id(&self, id: u32, languages: &[Language]) -> Result<Option<Role>, Box<dyn Error>> {
    let id = id as i32;
    let db_languages: Vec<DbLanguage> = languages.iter().map(|x| (*x).into()).collect();

    let roles = Select::new::<DbRole>()
      .columns_table::<DbRole>()
      .distinct_on(DbRole::TABLE_NAME, "id")
      .inner_join::<DbRoleTranslation>(
        None,
        Expression::new(ValueEqual::new((DbRole::TABLE_NAME, "id"), id)).and(role_id_equal_fk_translation()),
      )
      .get_single_destruct(self.client)
      .await?;
    let Some(item) = roles else {
      return Ok(None);
    };
    let mut translations = Select::new::<DbRoleTranslation>()
      .columns::<DbRoleTranslation>(DbRoleTranslation::TABLE_NAME)
      .where_expression(
        Expression::value_equal(DbRoleTranslation::TABLE_NAME, "fktranslation", item.id)
          .and(in_languages(&db_languages)),
      )
      .query_destruct(self.client)
      .await?;
    let items = [item];
    let no_translations: Vec<i32> = no_translation_ids(&items, &translations);
    let [item] = items;

    let mut extra_translations = Select::new::<DbRoleTranslation>()
      .distinct_on(DbRoleTranslation::TABLE_NAME, "fktranslation")
      .columns::<DbRoleTranslation>(DbRoleTranslation::TABLE_NAME)
      .where_expression(fk_translation_in_ids(&no_translations))
      .query_destruct(self.client)
      .await?;

    translations.append(&mut extra_translations);

    let translations: Vec<(Language, RoleTranslation)> = translations
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

  async fn get_by_ids(&self, ids: &[u32], languages: &[Language]) -> Result<Vec<Role>, Box<dyn Error>> {
    let ids = to_i32(ids);
    let db_languages: Vec<DbLanguage> = languages.iter().map(|x| (*x).into()).collect();

    let roles = Select::new::<DbRole>()
      .columns_table::<DbRole>()
      .distinct_on(DbRole::TABLE_NAME, "id")
      .transform(inner_join_translation)
      .where_expression(id_in_ids(&ids))
      .query_destruct(self.client)
      .await?;

    if roles.is_empty() {
      return Ok(vec![]);
    }
    let role_ids: Vec<i32> = roles.iter().map(|x| x.id).collect();

    let mut translations = role_translation_select(&role_ids, &db_languages)
      .query_destruct(self.client)
      .await?;

    let no_translations: Vec<i32> = no_translation_ids(&roles, &translations);

    let mut extra_translations = Select::new::<DbRoleTranslation>()
      .distinct_on(DbRoleTranslation::TABLE_NAME, "fktranslation")
      .columns::<DbRoleTranslation>(DbRoleTranslation::TABLE_NAME)
      .where_expression(fk_translation_in_ids(&no_translations))
      .query_destruct(self.client)
      .await?;

    translations.append(&mut extra_translations);

    let translations: Vec<(Language, i32, RoleTranslation)> = translations
      .into_iter()
      .map(|x| (x.language.into(), x.fk_translation, x.to_entity()))
      .collect();

    let translations = map_translation(&roles, translations);
    let available = self.available_languages(&role_ids).await?;
    let roles = to_entities(roles, available, translations);
    Ok(roles)
  }

  async fn get_by_name(
    &self,
    name: &str,
    languages: &[Language],
    pagination: Pagination,
  ) -> Result<ItemsTotal<Role>, Box<dyn Error>> {
    let name = format!("%{name}%");

    let db_languages: Vec<DbLanguage> = languages.iter().map(|x| (*x).into()).collect();
    let total = Select::new::<DbRole>()
      .transform(|x| inner_join_translation_on_name(x, &name, &db_languages))
      .query_count(self.client)
      .await? as usize;

    let roles = Select::new::<DbRole>()
      .columns_table::<DbRole>()
      .distinct_on(DbRole::TABLE_NAME, "id")
      .transform(|x| inner_join_translation_on_name(x, &name, &db_languages))
      .pagination(pagination)
      .query_destruct(self.client)
      .await?;
    let role_ids: Vec<i32> = roles.iter().map(|x| x.id).collect();

    let mut translations = role_translation_select(&role_ids, &db_languages)
      .where_expression(role_translation_with_name(&name))
      .query_destruct(self.client)
      .await?;

    let no_translations: Vec<i32> = no_translation_ids(&roles, &translations);

    let mut extra_translations = Select::new::<DbRoleTranslation>()
      .distinct_on(DbRoleTranslation::TABLE_NAME, "fktranslation")
      .columns::<DbRoleTranslation>(DbRoleTranslation::TABLE_NAME)
      .where_expression(fk_translation_in_ids(&no_translations))
      .query_destruct(self.client)
      .await?;

    translations.append(&mut extra_translations);

    let translations: Vec<(Language, i32, RoleTranslation)> = translations
      .into_iter()
      .map(|x| (x.language.into(), x.fk_translation, x.to_entity()))
      .collect();

    let translations = map_translation(&roles, translations);
    let available = self.available_languages(&role_ids).await?;
    let roles = to_entities(roles, available, translations);
    Ok(ItemsTotal { items: roles, total })
  }

  async fn filter_existing(&self, roles: &[u32]) -> Result<Vec<u32>, Box<dyn Error>> {
    let roles = to_i32(roles);

    let count = Select::new::<DbRole>()
      .column::<i32>(DbRole::TABLE_NAME, "id")
      .where_expression(Expression::new(ValueIn::new((DbRole::TABLE_NAME, "id"), &roles)))
      .query(self.client)
      .await?
      .into_iter()
      .map(|x| x.0 as u32)
      .collect();
    Ok(count)
  }
}
impl DefaultRoleRepository<'_> {
  async fn available_languages(&self, ids: &[i32]) -> Result<HashMap<i32, Vec<Language>>, Box<dyn Error>> {
    let available_translations = Select::new::<DbRoleTranslation>()
      .column::<i32>(DbRoleTranslation::TABLE_NAME, "fktranslation")
      .column::<DbLanguage>(DbRoleTranslation::TABLE_NAME, "language")
      .where_expression(Expression::new(ValueIn::new(
        (DbRoleTranslation::TABLE_NAME, "fktranslation"),
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
fn role_translation_select<'a>(
  role_ids: &'a [i32],
  db_languages: &'a [DbLanguage],
) -> Select<'a, (DbRoleTranslation,)> {
  Select::new::<DbRoleTranslation>()
    .columns::<DbRoleTranslation>(DbRoleTranslation::TABLE_NAME)
    .where_expression(fk_translation_in_ids(role_ids))
    .where_expression(in_languages(db_languages))
}
fn map_translation(
  roles: &[DbRole],
  translations: Vec<(Language, i32, RoleTranslation)>,
) -> HashMap<i32, Vec<(Language, RoleTranslation)>> {
  let mut new_translations: HashMap<i32, Vec<(Language, RoleTranslation)>> = HashMap::new();
  for role in roles {
    new_translations.insert(role.id, vec![]);
  }
  for (language, id, translation) in translations {
    let result = new_translations.get_mut(&id).unwrap();
    result.push((language, translation));
  }
  new_translations
}
fn role_id_equal_fk_translation<'a>() -> Expression<'a> {
  Expression::column_equal(
    (DbRole::TABLE_NAME, "id"),
    (DbRoleTranslation::TABLE_NAME, "fktranslation"),
  )
}
fn role_translation_with_name(name: &String) -> Expression {
  Expression::new(ValueILike::new((DbRoleTranslation::TABLE_NAME, "name"), name))
}
fn inner_join_translation_on_name<'a, T: FromRow<DbType = T> + CombinedType>(
  select: Select<'a, T>,
  name: &'a String,
  db_languages: &'a [DbLanguage],
) -> Select<'a, T> {
  select.inner_join::<DbRoleTranslation>(
    None,
    Expression::value_i_like((DbRoleTranslation::TABLE_NAME, "name"), name)
      .and(in_languages(db_languages))
      .and(role_id_equal_fk_translation()),
  )
}
fn to_entities(
  roles: Vec<DbRole>,
  mut available: HashMap<i32, Vec<Language>>,
  mut translations: HashMap<i32, Vec<(Language, RoleTranslation)>>,
) -> Vec<Role> {
  roles
    .into_iter()
    .map(|role| {
      let id = role.id;
      role.to_entity(AvailableTranslations {
        available_languages: available.remove(&id).unwrap(),
        translations: HashMap::from_iter(translations.remove(&id).unwrap()),
      })
    })
    .collect()
}
fn inner_join_translation<T: FromRow<DbType = T> + CombinedType>(select: Select<T>) -> Select<T> {
  select.inner_join::<DbRoleTranslation>(None, role_id_equal_fk_translation())
}
fn in_languages(languages: &[DbLanguage]) -> Expression {
  Expression::new(ValueIn::new((DbRoleTranslation::TABLE_NAME, "language"), languages))
}
fn fk_translation_in_ids(ids: &[i32]) -> Expression {
  Expression::new(ValueIn::new((DbRoleTranslation::TABLE_NAME, "fktranslation"), ids))
}
fn id_in_ids(ids: &[i32]) -> Expression {
  Expression::new(ValueIn::new((DbRole::TABLE_NAME, "id"), ids))
}
fn no_translation_ids(role_ids: &[DbRole], translations: &[DbRoleTranslation]) -> Vec<i32> {
  role_ids
    .iter()
    .filter_map(|x| {
      translations
        .iter()
        .find(|y| y.fk_translation == x.id)
        .map_or(Some(x.id), |_| None)
    })
    .collect()
}
