use std::error::Error;
use std::sync::Arc;

use async_trait::async_trait;
use tokio_postgres::Transaction;

use domain::entities::franchise::Franchise;
use domain::entities::franchise::create_partial_franchise::CreatePartialFranchise;
use domain::enums::language::Language;
use from_row::Table;
use repositories::franchise_repository::FranchiseRepository;
use repositories::franchise_repository::mut_franchise_repository::MutFranchiseRepository;

use crate::convert_to_sql::to_i32;
use crate::delete::Delete;
use crate::enums::db_language::DbLanguage;
use crate::insert::Insert;
use crate::schemas::db_franchise::DbFranchise;
use crate::schemas::db_franchise_translation::DbFranchiseTranslation;
use crate::select::conditions::value_in::ValueIn;
use crate::select::expression::Expression;

pub struct DefaultMutFranchiseRepository<'a> {
  transaction: &'a Transaction<'a>,
  franchise_repository: Arc<dyn FranchiseRepository + 'a>,
}

impl<'a> DefaultMutFranchiseRepository<'a> {
  pub fn new(
    transaction: &'a Transaction<'a>,
    franchise_repository: Arc<dyn FranchiseRepository + 'a>,
  ) -> DefaultMutFranchiseRepository<'a> {
    DefaultMutFranchiseRepository {
      transaction,
      franchise_repository,
    }
  }
}

#[async_trait]
impl MutFranchiseRepository for DefaultMutFranchiseRepository<'_> {
  async fn create(&self, item: CreatePartialFranchise) -> Result<Franchise, Box<dyn Error>> {
    let id = self.insert_franchise(&item).await? as u32;
    self.insert_translation(&item, id).await?;
    let languages: Vec<Language> = item.translations.keys().copied().collect();
    let franchise = self
      .franchise_repository
      .get_by_id(id, &languages)
      .await?
      .expect("Franchise was just created");
    Ok(franchise)
  }

  async fn delete(&self, ids: &[u32]) -> Result<(), Box<dyn Error>> {
    let ids = to_i32(ids);

    Delete::new::<DbFranchiseTranslation>(fk_translation_in_ids(&ids))
      .execute_transaction(self.transaction)
      .await?;

    Delete::new::<DbFranchise>(franchise_id_in_ids(&ids))
      .execute_transaction(self.transaction)
      .await?;
    Ok(())
  }
}

impl DefaultMutFranchiseRepository<'_> {
  async fn insert_franchise(&self, _item: &CreatePartialFranchise) -> Result<i32, Box<dyn Error>> {
    let id = Insert::new::<DbFranchise>([])
      .returning_transaction("id", self.transaction)
      .await?;
    Ok(id)
  }
  async fn insert_translation(&self, item: &CreatePartialFranchise, id: u32) -> Result<(), Box<dyn Error>> {
    let id = id as i32;
    let translations: Vec<(&String, DbLanguage)> = item
      .translations
      .iter()
      .map(|x| (&x.1.name, DbLanguage::from(*x.0)))
      .collect();

    let mut insert = Insert::new::<DbFranchiseTranslation>(["name", "fktranslation", "language"]);
    for (title, language) in &translations {
      insert.values_ref([*title, &id, language]);
    }
    insert.execute_transaction(self.transaction).await?;
    Ok(())
  }
}
fn fk_translation_in_ids(ids: &[i32]) -> Expression<'_> {
  Expression::new(ValueIn::new((DbFranchiseTranslation::TABLE_NAME, "fktranslation"), ids))
}
fn franchise_id_in_ids(ids: &[i32]) -> Expression<'_> {
  Expression::new(ValueIn::new((DbFranchise::TABLE_NAME, "id"), ids))
}
