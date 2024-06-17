use std::error::Error;
use std::sync::Arc;

use async_trait::async_trait;
use tokio_postgres::Client;

use domain::entities::book::book_character::BookCharacter;
use domain::enums::language::Language;
use domain::items_total::ItemsTotal;
use domain::pagination::Pagination;
use from_row::Table;
use repositories::book_repository::book_character_repository::BookCharacterRepository;
use repositories::book_repository::BookRepository;
use repositories::character_repository::CharacterRepository;

use crate::enums::db_language::DbLanguage;
use crate::schemas::db_book_character::DbBookCharacter;
use crate::select::comparison::Comparison::Equal;
use crate::select::condition::Condition::Value;
use crate::select::expression::Expression;
use crate::select::Select;

pub struct DefaultBookCharacterRepository<'a> {
  client: &'a Client,
  default_language: DbLanguage,
  book_repository: Arc<dyn BookRepository + 'a>,
  character_repository: Arc<dyn CharacterRepository + 'a>,
}

impl<'a> DefaultBookCharacterRepository<'a> {
  pub fn new(client: &'a Client,
             default_language: Language,
             book_repository: Arc<dyn BookRepository + 'a>,
             character_repository: Arc<dyn CharacterRepository + 'a>,
  ) -> DefaultBookCharacterRepository<'a> {
    DefaultBookCharacterRepository {
      client,
      default_language: default_language.into(),
      book_repository,
      character_repository,
    }
  }
}

#[async_trait]
impl<'a> BookCharacterRepository for DefaultBookCharacterRepository<'a> {
  async fn get(&self, book_id: u32, language: Language, pagination: Pagination) -> Result<ItemsTotal<BookCharacter>, Box<dyn Error>> {
    let book_id = book_id as i32;

    let total = Select::new::<DbBookCharacter>()
      .where_expression(Expression::new(Value((DbBookCharacter::TABLE_NAME, "fkbook"), Equal(&book_id))))
      .count()
      .get_single(self.client).await?
      .expect("Count should return one row");
    let total = total.0 as usize;

    let character_books_ids = Select::new::<DbBookCharacter>()
      .column::<i32>(DbBookCharacter::TABLE_NAME, "fkcharacter")
      .where_expression(Expression::new(Value((DbBookCharacter::TABLE_NAME, "fkbook"), Equal(&book_id))))
      .pagination(pagination)
      .query(self.client)
      .await?;

    if character_books_ids.is_empty() {
      return Ok(ItemsTotal {
        items: vec![],
        total,
      });
    }

    let character_ids: Vec<u32> = character_books_ids
      .iter()
      .map(|x| x.0 as u32)
      .collect();

    let characters = self.character_repository.get_by_ids(&character_ids, language).await?;

    let items = characters
      .into_iter()
      .map(|character| BookCharacter { character })
      .collect();

    Ok(ItemsTotal {
      items,
      total,
    })
  }
}
