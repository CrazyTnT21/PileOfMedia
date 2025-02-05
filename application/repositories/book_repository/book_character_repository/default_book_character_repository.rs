use std::error::Error;
use std::sync::Arc;

use async_trait::async_trait;
use domain::entities::book::book_character::BookCharacter;
use domain::enums::language::Language;
use from_row::Table;
use repositories::book_repository::book_character_repository::BookCharacterRepository;
use repositories::character_repository::CharacterRepository;
use tokio_postgres::Client;

use crate::convert_to_sql::to_i32;
use crate::schemas::db_book_character::DbBookCharacter;
use crate::select::conditions::value_equal::ValueEqual;
use crate::select::conditions::value_in::ValueIn;
use crate::select::expression::Expression;
use crate::select::Select;

pub struct DefaultBookCharacterRepository<'a> {
  client: &'a Client,
  character_repository: Arc<dyn CharacterRepository + 'a>,
}

impl<'a> DefaultBookCharacterRepository<'a> {
  pub fn new(
    client: &'a Client,
    character_repository: Arc<dyn CharacterRepository + 'a>,
  ) -> DefaultBookCharacterRepository<'a> {
    DefaultBookCharacterRepository {
      client,
      character_repository,
    }
  }
}

#[async_trait]
impl BookCharacterRepository for DefaultBookCharacterRepository<'_> {
  async fn get(&self, book_id: u32, languages: &[Language]) -> Result<Vec<BookCharacter>, Box<dyn Error>> {
    let book_id = book_id as i32;

    let character_ids: Vec<u32> = Select::new::<DbBookCharacter>()
      .column::<i32>(DbBookCharacter::TABLE_NAME, "fkcharacter")
      .where_expression(Expression::new(ValueEqual::new(
        (DbBookCharacter::TABLE_NAME, "fkbook"),
        book_id,
      )))
      .query(self.client)
      .await?
      .into_iter()
      .map(|x| x.0 as u32)
      .collect();
    let items = self
      .character_repository
      .get_by_ids(&character_ids, languages)
      .await?
      .into_iter()
      .map(|x| BookCharacter { character: x })
      .collect();
    Ok(items)
  }

  async fn filter_existing(&self, book_id: u32, characters: &[u32]) -> Result<Vec<u32>, Box<dyn Error>> {
    let book_id = book_id as i32;
    let characters = to_i32(characters);

    let filtered = Select::new::<DbBookCharacter>()
      .column::<i32>(DbBookCharacter::TABLE_NAME, "fkcharacter")
      .where_expression(Expression::new(ValueIn::new(
        (DbBookCharacter::TABLE_NAME, "fkcharacter"),
        &characters,
      )))
      .where_expression(Expression::value_equal(DbBookCharacter::TABLE_NAME, "fkbook", book_id))
      .query(self.client)
      .await?
      .into_iter()
      .map(|x| x.0 as u32)
      .collect();
    Ok(filtered)
  }
}
