use std::error::Error;

use async_trait::async_trait;
use tokio_postgres::Client;

use domain::entities::franchise::Franchise;
use domain::enums::language::Language;
use domain::items_total::ItemsTotal;
use domain::pagination::Pagination;
use from_row::{FromRow, Table};
use repositories::franchise_repository::FranchiseRepository;

use crate::convert_to_sql::{to_i32};
use crate::enums::db_language::DbLanguage;
use crate::fallback_unwrap::fallback_unwrap;
use crate::schemas::db_franchise::DbFranchise;
use crate::schemas::db_franchise_translation::DbFranchiseTranslation;
use crate::select::combined_tuple::CombinedType;
use crate::select::conditions::column_equal::ColumnEqual;
use crate::select::conditions::column_null::ColumnNull;
use crate::select::conditions::value_ilike::ValueILike;
use crate::select::conditions::value_equal::ValueEqual;
use crate::select::conditions::value_in::ValueIn;
use crate::select::expression::Expression;
use crate::select::Select;

pub struct DefaultFranchiseRepository<'a> {
  client: &'a Client,
  default_language: DbLanguage,
}

impl<'a> DefaultFranchiseRepository<'a> {
  pub fn new(client: &'a Client, language: Language) -> DefaultFranchiseRepository<'a> {
    DefaultFranchiseRepository { client, default_language: language.into() }
  }
}

#[async_trait]
impl<'a> FranchiseRepository for DefaultFranchiseRepository<'a> {
  async fn get(&self, language: Language, pagination: Pagination) -> Result<ItemsTotal<Franchise>, Box<dyn Error>> {
    let language = DbLanguage::from(language);

    let total = Select::new::<DbFranchise>()
      .transform(|x| self.franchise_joins(x, &language))
      .count()
      .get_single(self.client)
      .await?
      .expect("Count should return one row");
    let total = total.0 as usize;

    let franchises = franchise_select_columns()
      .transform(|x| self.franchise_joins(x, &language))
      .pagination(pagination)
      .query(self.client)
      .await?
      .into_iter()
      .map(to_entity)
      .collect();

    Ok(ItemsTotal { items: franchises, total })
  }

  async fn get_by_id(&self, id: u32, language: Language) -> Result<Option<Franchise>, Box<dyn Error>> {
    let id = id as i32;
    let language = DbLanguage::from(language);
    let franchise = franchise_select_columns()
      .transform(|x| self.franchise_joins(x, &language))
      .where_expression(Expression::new(ValueEqual::new(("franchise", "id"), id)))
      .get_single(self.client)
      .await?;
    Ok(franchise.map(to_entity))
  }

  async fn get_by_ids(&self, ids: &[u32], language: Language) -> Result<Vec<Franchise>, Box<dyn Error>> {
    let language = DbLanguage::from(language);
    let ids = to_i32(ids);

    let franchises = franchise_select_columns()
      .transform(|x| self.franchise_joins(x, &language))
      .where_expression(Expression::new(ValueIn::new(("franchise", "id"), &ids)))
      .query(self.client)
      .await?
      .into_iter()
      .map(to_entity)
      .collect();

    Ok(franchises)
  }

  async fn get_by_name(&self, name: &str, language: Language, pagination: Pagination) -> Result<ItemsTotal<Franchise>, Box<dyn Error>> {
    let language = DbLanguage::from(language);
    let name = format!("%{name}%");

    let total = Select::new::<DbFranchise>()
      .transform(|x| self.franchise_joins(x, &language))
      .where_expression(Expression::new(ValueILike::new(("franchise_translation", "name"), &name))
        .or(Expression::new(ValueILike::new(("franchise_translation_fallback", "name"), &name))))
      .count()
      .get_single(self.client)
      .await?
      .expect("Count should return one row");

    let total = total.0 as usize;
    let franchises = franchise_select_columns()
      .transform(|x| self.franchise_joins(x, &language))
      .where_expression(Expression::new(ValueILike::new(("franchise_translation", "name"), &name))
        .or(Expression::new(ValueILike::new(("franchise_translation_fallback", "name"), &name))))
      .pagination(pagination)
      .query(self.client)
      .await?
      .into_iter()
      .map(to_entity)
      .collect();
    Ok(ItemsTotal { items: franchises, total })
  }

  async fn filter_existing(&self, franchises: &[u32]) -> Result<Vec<u32>, Box<dyn Error>> {
    let franchises = to_i32(franchises);

    let count = Select::new::<DbFranchise>()
      .column::<i32>(DbFranchise::TABLE_NAME, "id")
      .where_expression(Expression::new(ValueIn::new((DbFranchise::TABLE_NAME, "id"), &franchises)))
      .query(self.client)
      .await?
      .into_iter()
      .map(|x| { x.0 as u32 })
      .collect();
    Ok(count)
  }
}

impl<'a> DefaultFranchiseRepository<'a> {
  fn franchise_joins<T: FromRow<DbType=T> + CombinedType>(&'a self, select: Select<'a, T>, language: &'a DbLanguage) -> Select<'a, T> {
    select
      .left_join::<DbFranchiseTranslation>(
        Some("franchise_translation"),
        Expression::column_equal("franchise_translation", "language", language)
          .and(Expression::new(ColumnEqual::new(("franchise_translation", "fktranslation"), ("franchise", "id")))),
      )
      .left_join::<DbFranchiseTranslation>(
        Some("franchise_translation_fallback"),
        Expression::column_equal("franchise_translation_fallback", "language", &self.default_language)
          .and(Expression::new(ColumnEqual::new(("franchise_translation_fallback", "fktranslation"), ("franchise", "id"))))
          .and(Expression::new(ColumnNull::new(("franchise_translation", "fktranslation")))),
      )
  }
}

fn to_entity(franchise: (DbFranchise, Option<DbFranchiseTranslation>, Option<DbFranchiseTranslation>)) -> Franchise {
  franchise.0.to_entity(fallback_unwrap(franchise.1, franchise.2))
}

fn franchise_select_columns<'a>() -> Select<'a, FranchiseColumns> {
  Select::new::<DbFranchise>()
    .columns::<DbFranchise>("franchise")
    .columns::<Option<DbFranchiseTranslation>>("franchise_translation")
    .columns::<Option<DbFranchiseTranslation>>("franchise_translation_fallback")
}

type FranchiseColumns = (DbFranchise, Option<DbFranchiseTranslation>, Option<DbFranchiseTranslation>);
