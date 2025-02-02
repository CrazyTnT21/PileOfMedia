use std::error::Error;

use async_trait::async_trait;
use tokio_postgres::Client;

use domain::entities::role::Role;
use domain::enums::language::Language;
use domain::items_total::ItemsTotal;
use domain::pagination::Pagination;
use from_row::{FromRow, Table};
use repositories::role_repository::RoleRepository;

use crate::convert_to_sql::to_i32;
use crate::enums::db_language::DbLanguage;
use crate::fallback_unwrap::fallback_unwrap;
use crate::schemas::db_role::DbRole;
use crate::schemas::db_role_translation::DbRoleTranslation;
use crate::select::combined_tuple::CombinedType;
use crate::select::conditions::column_equal::ColumnEqual;
use crate::select::conditions::column_null::ColumnNull;
use crate::select::conditions::value_equal::ValueEqual;
use crate::select::conditions::value_ilike::ValueILike;
use crate::select::conditions::value_in::ValueIn;
use crate::select::expression::Expression;
use crate::select::Select;

pub struct DefaultRoleRepository<'a> {
  client: &'a Client,
  default_language: DbLanguage,
}

impl<'a> DefaultRoleRepository<'a> {
  pub fn new(client: &'a Client, language: Language) -> DefaultRoleRepository<'a> {
    DefaultRoleRepository {
      client,
      default_language: language.into(),
    }
  }
}

#[async_trait]
impl RoleRepository for DefaultRoleRepository<'_> {
  async fn get(&self, language: Language, pagination: Pagination) -> Result<ItemsTotal<Role>, Box<dyn Error>> {
    let language = DbLanguage::from(language);

    let total = Select::new::<DbRole>()
      .transform(|x| self.role_joins(x, &language))
      .query_count(self.client)
      .await? as usize;

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
      .where_expression(Expression::new(ValueEqual::new(("role", "id"), id)))
      .get_single(self.client)
      .await?;

    Ok(role.map(to_entity))
  }

  async fn get_by_ids(&self, ids: &[u32], language: Language) -> Result<Vec<Role>, Box<dyn Error>> {
    let language = DbLanguage::from(language);
    let ids = to_i32(ids);

    let roles = role_select_columns()
      .transform(|x| self.role_joins(x, &language))
      .where_expression(Expression::new(ValueIn::new(("role", "id"), &ids)))
      .query(self.client)
      .await?
      .into_iter()
      .map(to_entity)
      .collect();

    Ok(roles)
  }

  async fn get_by_name(
    &self,
    name: &str,
    language: Language,
    pagination: Pagination,
  ) -> Result<ItemsTotal<Role>, Box<dyn Error>> {
    let language = DbLanguage::from(language);
    let name = format!("%{name}%");

    let total = Select::new::<DbRole>()
      .transform(|x| self.role_joins(x, &language))
      .where_expression(
        Expression::new(ValueILike::new(("role_translation", "name"), &name)).or(Expression::new(ValueILike::new(
          ("role_translation_fallback", "name"),
          &name,
        ))),
      )
      .query_count(self.client)
      .await? as usize;

    let roles = role_select_columns()
      .transform(|x| self.role_joins(x, &language))
      .where_expression(
        Expression::new(ValueILike::new(("role_translation", "name"), &name)).or(Expression::new(ValueILike::new(
          ("role_translation_fallback", "name"),
          &name,
        ))),
      )
      .pagination(pagination)
      .query(self.client)
      .await?
      .into_iter()
      .map(to_entity)
      .collect();
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

impl<'a> DefaultRoleRepository<'a> {
  fn role_joins<T: FromRow<DbType = T> + CombinedType>(
    &'a self,
    select: Select<'a, T>,
    language: &'a DbLanguage,
  ) -> Select<'a, T> {
    select
      .left_join::<DbRoleTranslation>(
        Some("role_translation"),
        Expression::value_equal("role_translation", "language", language).and(Expression::new(ColumnEqual::new(
          ("role_translation", "fktranslation"),
          ("role", "id"),
        ))),
      )
      .left_join::<DbRoleTranslation>(
        Some("role_translation_fallback"),
        Expression::value_equal("role_translation_fallback", "language", self.default_language)
          .and(Expression::new(ColumnEqual::new(
            ("role_translation_fallback", "fktranslation"),
            ("role", "id"),
          )))
          .and(Expression::new(ColumnNull::new(("role_translation", "fktranslation")))),
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
