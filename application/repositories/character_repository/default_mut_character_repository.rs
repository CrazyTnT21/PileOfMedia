use std::error::Error;
use std::sync::Arc;

use async_trait::async_trait;
use tokio_postgres::Transaction;

use domain::entities::character::Character;
use domain::entities::character::create_partial_character::{
  CreatePartialCharacter, CreatePartialCharacterTranslation,
};
use domain::enums::language::Language;
use from_row::Table;
use repositories::character_repository::CharacterRepository;
use repositories::character_repository::mut_character_repository::MutCharacterRepository;

use crate::convert_to_sql::to_i32;
use crate::delete::Delete;
use crate::enums::db_language::DbLanguage;
use crate::insert::Insert;
use crate::schemas::db_character::DbCharacter;
use crate::schemas::db_character_translation::DbCharacterTranslation;
use crate::select::conditions::value_in::ValueIn;
use crate::select::expression::Expression;

pub struct DefaultMutCharacterRepository<'a> {
  transaction: &'a Transaction<'a>,
  character_repository: Arc<dyn CharacterRepository + 'a>,
}

impl<'a> DefaultMutCharacterRepository<'a> {
  pub fn new(
    transaction: &'a Transaction<'a>,
    character_repository: Arc<dyn CharacterRepository + 'a>,
  ) -> DefaultMutCharacterRepository<'a> {
    DefaultMutCharacterRepository {
      transaction,
      character_repository,
    }
  }
}

#[async_trait]
impl MutCharacterRepository for DefaultMutCharacterRepository<'_> {
  async fn create(&self, item: CreatePartialCharacter) -> Result<Character, Box<dyn Error>> {
    let id = self.insert_character(&item).await? as u32;
    self.insert_translation(&item, id).await?;

    let languages: Vec<Language> = item.translations.keys().copied().collect();
    let character = self
      .character_repository
      .get_by_id(id, &languages)
      .await?
      .expect("Character was just created");
    Ok(character)
  }

  async fn delete(&self, ids: &[u32]) -> Result<(), Box<dyn Error>> {
    let ids = to_i32(ids);

    Delete::new::<DbCharacterTranslation>(Expression::new(ValueIn::new(
      (DbCharacterTranslation::TABLE_NAME, "translation_id"),
      &ids,
    )))
    .execute_transaction(self.transaction)
    .await?;

    Delete::new::<DbCharacter>(Expression::new(ValueIn::new((DbCharacter::TABLE_NAME, "id"), &ids)))
      .execute_transaction(self.transaction)
      .await?;
    Ok(())
  }
}

impl DefaultMutCharacterRepository<'_> {
  async fn insert_character(&self, item: &CreatePartialCharacter) -> Result<i32, Box<dyn Error>> {
    let image_id = item.image.as_ref().map(|x| x.id as i32);
    let id = Insert::new::<DbCharacter>(["birthday", "height", "image_id"])
      .values([&item.birthday, &item.height_cm.map(|x| x as i32), &image_id])
      .returning_transaction("id", self.transaction)
      .await?;
    Ok(id)
  }
  async fn insert_translation(&self, item: &CreatePartialCharacter, id: u32) -> Result<(), Box<dyn Error>> {
    let id = id as i32;
    let translations: Vec<(DbLanguage, &CreatePartialCharacterTranslation)> = item
      .translations
      .iter()
      .map(|(language, item)| (DbLanguage::from(*language), item))
      .collect();

    let mut insert = Insert::new::<DbCharacterTranslation>([
      "description",
      "translation_id",
      "language",
      "name",
      "first_name",
      "last_name",
    ]);
    for (language, item) in &translations {
      insert.values_ref([
        &item.description,
        &id,
        language,
        &item.name,
        &item.first_name,
        &item.last_name,
      ]);
    }
    insert.execute_transaction(self.transaction).await?;
    Ok(())
  }
}
