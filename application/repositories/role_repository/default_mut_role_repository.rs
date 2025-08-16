use std::error::Error;
use std::sync::Arc;

use async_trait::async_trait;
use tokio_postgres::Transaction;

use domain::entities::role::Role;
use domain::entities::role::create_partial_role::CreatePartialRole;
use domain::enums::language::Language;
use from_row::Table;
use repositories::role_repository::RoleRepository;
use repositories::role_repository::mut_role_repository::MutRoleRepository;

use crate::convert_to_sql::to_i32;
use crate::delete::Delete;
use crate::enums::db_language::DbLanguage;
use crate::insert::Insert;
use crate::schemas::db_role::DbRole;
use crate::schemas::db_role_translation::DbRoleTranslation;
use crate::select::conditions::value_in::ValueIn;
use crate::select::expression::Expression;

pub struct DefaultMutRoleRepository<'a> {
  transaction: &'a Transaction<'a>,
  role_repository: Arc<dyn RoleRepository + 'a>,
}

impl<'a> DefaultMutRoleRepository<'a> {
  pub fn new(
    transaction: &'a Transaction<'a>,
    role_repository: Arc<dyn RoleRepository + 'a>,
  ) -> DefaultMutRoleRepository<'a> {
    DefaultMutRoleRepository {
      transaction,
      role_repository,
    }
  }
}

#[async_trait]
impl MutRoleRepository for DefaultMutRoleRepository<'_> {
  async fn create(&self, item: CreatePartialRole) -> Result<Role, Box<dyn Error>> {
    let id = self.insert_role(&item).await? as u32;
    self.insert_translation(&item, id).await?;
    let languages: Vec<Language> = item.translations.keys().copied().collect();
    let role = self
      .role_repository
      .get_by_id(id, &languages)
      .await?
      .expect("Role was just created");
    Ok(role)
  }

  async fn delete(&self, ids: &[u32]) -> Result<(), Box<dyn Error>> {
    let ids = to_i32(ids);

    Delete::new::<DbRoleTranslation>(fk_translation_in_ids(&ids))
      .execute_transaction(self.transaction)
      .await?;

    Delete::new::<DbRole>(role_id_in_ids(&ids))
      .execute_transaction(self.transaction)
      .await?;
    Ok(())
  }
}

impl DefaultMutRoleRepository<'_> {
  async fn insert_role(&self, _item: &CreatePartialRole) -> Result<i32, Box<dyn Error>> {
    let id = Insert::new::<DbRole>([])
      .returning_transaction("id", self.transaction)
      .await?;
    Ok(id)
  }
  async fn insert_translation(&self, item: &CreatePartialRole, id: u32) -> Result<(), Box<dyn Error>> {
    let id = id as i32;
    let translations: Vec<(&String, DbLanguage)> = item
      .translations
      .iter()
      .map(|x| (&x.1.name, DbLanguage::from(*x.0)))
      .collect();

    let mut insert = Insert::new::<DbRoleTranslation>(["name", "fktranslation", "language"]);
    for (title, language) in &translations {
      insert.values_ref([*title, &id, language]);
    }
    insert.execute_transaction(self.transaction).await?;
    Ok(())
  }
}
fn fk_translation_in_ids(ids: &[i32]) -> Expression<'_> {
  Expression::new(ValueIn::new((DbRoleTranslation::TABLE_NAME, "fktranslation"), ids))
}
fn role_id_in_ids(ids: &[i32]) -> Expression<'_> {
  Expression::new(ValueIn::new((DbRole::TABLE_NAME, "id"), ids))
}
