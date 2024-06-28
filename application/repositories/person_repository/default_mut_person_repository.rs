use std::error::Error;
use std::sync::Arc;

use async_trait::async_trait;
use tokio_postgres::Transaction;

use domain::entities::person::Person;
use domain::entities::person::create_partial_person::CreatePartialPerson;
use domain::enums::language::Language;
use from_row::Table;
use repositories::person_repository::PersonRepository;
use repositories::person_repository::mut_person_repository::MutPersonRepository;

use crate::convert_to_sql::to_i32;
use crate::delete::Delete;
use crate::enums::db_language::DbLanguage;
use crate::insert::Insert;
use crate::schemas::db_person::DbPerson;
use crate::schemas::db_person_translation::DbPersonTranslation;
use crate::select::conditions::value_in::ValueIn;
use crate::select::expression::Expression;

pub struct DefaultMutPersonRepository<'a> {
  transaction: &'a Transaction<'a>,
  default_language: Language,
  person_repository: Arc<dyn PersonRepository + 'a>,
}

impl<'a> DefaultMutPersonRepository<'a> {
  pub fn new(transaction: &'a Transaction<'a>,
             default_language: Language,
             person_repository: Arc<dyn PersonRepository + 'a>, ) -> DefaultMutPersonRepository<'a> {
    DefaultMutPersonRepository {
      transaction,
      default_language,
      person_repository,
    }
  }
}

#[async_trait]
impl MutPersonRepository for DefaultMutPersonRepository<'_> {
  async fn create(&self, item: CreatePartialPerson) -> Result<Person, Box<dyn Error>> {
    let id = self.insert_person(&item).await? as u32;
    self.insert_translation(&item, id).await?;

    let person = self.person_repository
      .get_by_id(id, self.default_language)
      .await?
      .expect("Person was just created");
    Ok(person)
  }

  async fn delete(&self, ids: &[u32]) -> Result<(), Box<dyn Error>> {
    let ids = to_i32(ids);

    Delete::new::<DbPersonTranslation>(Expression::new(ValueIn::new((DbPersonTranslation::TABLE_NAME, "fktranslation"), &ids)))
      .execute_transaction(self.transaction)
      .await?;

    Delete::new::<DbPerson>(Expression::new(ValueIn::new((DbPerson::TABLE_NAME, "id"), &ids)))
      .execute_transaction(self.transaction)
      .await?;
    Ok(())
  }
}

impl DefaultMutPersonRepository<'_> {
  async fn insert_person(&self, item: &CreatePartialPerson) -> Result<i32, Box<dyn Error>> {
    let image_id = item.image.as_ref().map(|x| x.id as i32);
    let id = Insert::new::<DbPerson>(["name", "firstname", "lastname", "birthday", "height", "fkimage"])
      .values([&item.name, &item.first_name, &item.last_name, &item.birthday, &item.height.map(|x| x as i16), &image_id])
      .returning_transaction("id", self.transaction).await?;
    Ok(id)
  }
  async fn insert_translation(&self, item: &CreatePartialPerson, id: u32) -> Result<(), Box<dyn Error>> {
    let id = id as i32;
    let translations: Vec<(&Option<String>, DbLanguage)> = item.translations
      .iter()
      .map(|x| (&x.1.description, DbLanguage::from(*x.0)))
      .collect();
    let mut insert = Insert::new::<DbPersonTranslation>(["description", "fktranslation", "language"]);
    for (description, language) in &translations {
      insert.values_ref([*description, &id, language]);
    }
    insert.execute_transaction(self.transaction).await?;
    Ok(())
  }
}
