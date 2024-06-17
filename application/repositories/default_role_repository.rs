use std::error::Error;

use async_trait::async_trait;
use tokio_postgres::Client;

use domain::entities::role::Role;
use domain::enums::language::Language;
use domain::items_total::ItemsTotal;
use domain::pagination::Pagination;
use from_row::{FromRow, Table};
use repositories::role_repository::RoleRepository;

use crate::convert_to_sql::{convert_to_sql, to_i32};
use crate::enums::db_language::DbLanguage;
use crate::fallback_unwrap::fallback_unwrap;
use crate::schemas::db_role::DbRole;
use crate::schemas::db_role_translation::DbRoleTranslation;
use crate::select::combined_tuple::CombinedType;
use crate::select::comparison::Comparison::{Equal, ILike, In};
use crate::select::condition::Condition::{Column, Value};
use crate::select::expression::Expression;
use crate::select::Select;

pub struct DefaultRoleRepository<'a> {
  client: &'a Client,
  default_language: DbLanguage,
}

impl<'a> DefaultRoleRepository<'a> {
  pub fn new(client: &'a Client, language: Language) -> DefaultRoleRepository<'a> {
    DefaultRoleRepository { client, default_language: language.into() }
  }
}

#[async_trait]
impl<'a> RoleRepository for DefaultRoleRepository<'a> {
  async fn get(&self, language: Language, pagination: Pagination) -> Result<ItemsTotal<Role>, Box<dyn Error>> {
    let language = DbLanguage::from(language);

    let total = Select::new::<DbRole>()
      .count()
      .transform(|x| self.role_joins(x, &language))
      .get_single(self.client).await?
      .expect("Count should return one row");
    let total = total.0 as usize;

    let roles = role_select_columns()
      .transform(|x| self.role_joins(x, &language))
      .pagination(pagination)
      .query(self.client)
      .await?
      .into_iter()
      .map(to_entity)
      .collect();

    Ok(ItemsTotal { items: roles, total })
  }

  async fn get_by_id(&self, id: u32, language: Language) -> Result<Option<Role>, Box<dyn Error>> {
    let id = id as i32;
    let language = DbLanguage::from(language);
    let role = role_select_columns()
      .transform(|x| self.role_joins(x, &language))
      .where_expression(Expression::new(Value(("role", "id"), Equal(&id))))
      .get_single(self.client)
      .await?;

    Ok(role.map(to_entity))
  }

  async fn get_by_ids(&self, ids: &[u32], language: Language) -> Result<Vec<Role>, Box<dyn Error>> {
    let language = DbLanguage::from(language);
    let ids = to_i32(ids);
    let ids = convert_to_sql(&ids);
    let roles = role_select_columns()
      .transform(|x| self.role_joins(x, &language))
      .where_expression(Expression::new(Value(("role", "id"), In(&ids))))
      .query(self.client)
      .await?
      .into_iter()
      .map(to_entity)
      .collect();

    Ok(roles)
  }

  async fn get_by_name(&self, name: &str, language: Language, pagination: Pagination) -> Result<ItemsTotal<Role>, Box<dyn Error>> {
    let language = DbLanguage::from(language);
    let name = format!("%{name}%");

    let total = Select::new::<DbRole>()
      .count()
      .transform(|x| self.role_joins(x, &language))
      .where_expression(Expression::new(Value(("role_translation", "name"), ILike(&name)))
        .or(Expression::new(Value(("role_translation_fallback", "name"), ILike(&name)))))
      .get_single(self.client).await?
      .expect("Count should return one row");
    let total = total.0 as usize;

    let roles = role_select_columns()
      .transform(|x| self.role_joins(x, &language))
      .where_expression(Expression::new(Value(("role_translation", "name"), ILike(&name)))
        .or(Expression::new(Value(("role_translation_fallback", "name"), ILike(&name)))))
      .pagination(pagination)
      .query(self.client)
      .await?
      .into_iter()
      .map(to_entity)
      .collect();
    Ok(ItemsTotal { items: roles, total })
  }
}

impl<'a> DefaultRoleRepository<'a> {
  fn role_joins<T: FromRow<DbType=T> + CombinedType>(&'a self, select: Select<'a, T>, language: &'a DbLanguage) -> Select<'a, T> {
    select
      .left_join::<DbRoleTranslation>(
        Some("role_translation"),
        Expression::column_equal("role_translation", "language", language)
          .and(Expression::new(Column(("role_translation", "fktranslation"), ("role", "id")))),
      )
      .left_join::<DbRoleTranslation>(
        Some("role_translation_fallback"),
        Expression::column_equal("role_translation_fallback", "language", &self.default_language)
          .and(Expression::new(Column(("role_translation_fallback", "fktranslation"), ("role", "id"))))
          .and(Expression::column_null("role_translation", "fktranslation")),
      )
  }
}

fn to_entity(role: (DbRole, Option<DbRoleTranslation>, Option<DbRoleTranslation>)) -> Role {
  role.0.to_entity(fallback_unwrap(role.1, role.2))
}

fn role_select_columns<'a>() -> Select<'a, RoleColumns> {
  Select::new::<DbRole>()
    .columns::<DbRole>(DbRole::TABLE_NAME)
    .columns::<Option<DbRoleTranslation>>("role_translation")
    .columns::<Option<DbRoleTranslation>>("role_translation_fallback")
}

type RoleColumns = (DbRole, Option<DbRoleTranslation>, Option<DbRoleTranslation>);
