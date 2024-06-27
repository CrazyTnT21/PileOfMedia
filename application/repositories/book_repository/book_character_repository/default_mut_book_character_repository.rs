use std::error::Error;

use async_trait::async_trait;
use tokio_postgres::Transaction;

use from_row::Table;
use repositories::book_repository::book_character_repository::mut_book_character_repository::MutBookCharacterRepository;

use crate::convert_to_sql::{to_i32};
use crate::delete::Delete;
use crate::insert::Insert;
use crate::schemas::db_book_character::DbBookCharacter;
use crate::select::conditions::value_in::ValueIn;
use crate::select::expression::Expression;

pub struct DefaultMutBookCharacterRepository<'a> {
  transaction: &'a Transaction<'a>,
}

impl<'a> DefaultMutBookCharacterRepository<'a> {
  pub fn new(transaction: &'a Transaction<'a>) -> DefaultMutBookCharacterRepository<'a> {
    DefaultMutBookCharacterRepository { transaction }
  }
}

#[async_trait]
impl<'a> MutBookCharacterRepository for DefaultMutBookCharacterRepository<'a> {
  async fn add(&self, book_id: u32, characters: &[u32]) -> Result<(), Box<dyn Error>> {
    let book_id = book_id as i32;
    let characters = to_i32(characters);
    let mut insert = Insert::new::<DbBookCharacter>(["fkbook", "fkcharacter"]);
    characters.iter().for_each(|x| { insert.push_as_ref([&book_id, x]); });
    insert.execute_transaction(self.transaction).await?;
    Ok(())
  }

  async fn remove(&self, book_id: u32, characters: &[u32]) -> Result<(), Box<dyn Error>> {
    let book_id = book_id as i32;
    let characters = to_i32(characters);

    Delete::new::<DbBookCharacter>(
      Expression::column_equal(DbBookCharacter::TABLE_NAME, "fkbook", book_id)
        .and(Expression::new(ValueIn::new((DbBookCharacter::TABLE_NAME, "fkcharacter"), &characters))))
      .execute_transaction(self.transaction)
      .await?;
    Ok(())
  }

  async fn remove_all(&self, book_ids: &[u32]) -> Result<(), Box<dyn Error>> {
    let book_ids = to_i32(book_ids);

    Delete::new::<DbBookCharacter>(
      Expression::new(ValueIn::new((DbBookCharacter::TABLE_NAME, "fkbook"), &book_ids)))
      .execute_transaction(self.transaction)
      .await?;
    Ok(())
  }
}
