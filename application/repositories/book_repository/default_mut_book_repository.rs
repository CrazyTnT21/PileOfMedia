use std::error::Error;
use std::sync::Arc;

use async_trait::async_trait;
use tokio_postgres::Transaction;

use domain::entities::book::create_partial_book::CreatePartialBook;
use domain::entities::book::Book;
use domain::enums::language::Language;
use from_row::Table;
use repositories::book_repository::book_character_repository::mut_book_character_repository::MutBookCharacterRepository;
use repositories::book_repository::book_genre_repository::mut_book_genre_repository::MutBookGenreRepository;
use repositories::book_repository::book_involved_repository::mut_book_involved_repository::MutBookInvolvedRepository;
use repositories::book_repository::book_theme_repository::mut_book_theme_repository::MutBookThemeRepository;
use repositories::book_repository::mut_book_repository::MutBookRepository;
use repositories::book_repository::BookRepository;

use crate::convert_to_sql::to_i32;
use crate::delete::Delete;
use crate::enums::db_language::DbLanguage;
use crate::insert::Insert;
use crate::schemas::db_book::DbBook;
use crate::schemas::db_book_statistic::DbBookStatistic;
use crate::schemas::db_book_translation::DbBookTranslation;
use crate::schemas::db_rating::DbRating;
use crate::select::conditions::value_in::ValueIn;
use crate::select::expression::Expression;
use crate::select::Select;

pub struct DefaultMutBookRepository<'a> {
  transaction: &'a Transaction<'a>,
  default_language: Language,
  mut_book_genre_repository: Arc<dyn MutBookGenreRepository + 'a>,
  mut_book_character_repository: Arc<dyn MutBookCharacterRepository + 'a>,
  mut_book_theme_repository: Arc<dyn MutBookThemeRepository + 'a>,
  mut_book_involved_repository: Arc<dyn MutBookInvolvedRepository + 'a>,
  book_repository: Arc<dyn BookRepository + 'a>,
}

impl<'a> DefaultMutBookRepository<'a> {
  pub fn new(
    transaction: &'a Transaction<'a>,
    default_language: Language,
    mut_book_genre_repository: Arc<dyn MutBookGenreRepository + 'a>,
    mut_book_character_repository: Arc<dyn MutBookCharacterRepository + 'a>,
    mut_book_theme_repository: Arc<dyn MutBookThemeRepository + 'a>,
    mut_book_involved_repository: Arc<dyn MutBookInvolvedRepository + 'a>,
    book_repository: Arc<dyn BookRepository + 'a>,
  ) -> DefaultMutBookRepository<'a> {
    DefaultMutBookRepository {
      transaction,
      default_language,
      mut_book_genre_repository,
      mut_book_character_repository,
      mut_book_theme_repository,
      mut_book_involved_repository,
      book_repository,
    }
  }
}

#[async_trait]
impl MutBookRepository for DefaultMutBookRepository<'_> {
  async fn create(&self, item: CreatePartialBook) -> Result<Book, Box<dyn Error>> {
    let id = self.insert_book(&item).await? as u32;
    self.insert_translation(&item, id).await?;
    self.insert_characters(&item, id).await?;
    self.insert_themes(&item, id).await?;
    self.insert_genres(&item, id).await?;
    self.insert_involved(&item, id).await?;

    let book = self
      .book_repository
      .get_by_id(id, self.default_language)
      .await?
      .expect("Book was just created");
    Ok(book)
  }

  async fn delete(&self, ids: &[u32]) -> Result<(), Box<dyn Error>> {
    self.mut_book_character_repository.remove_all(ids).await?;
    self.mut_book_genre_repository.remove_all(ids).await?;
    self.mut_book_involved_repository.remove_all(ids).await?;
    self.mut_book_theme_repository.remove_all(ids).await?;
    let ids = to_i32(ids);

    Delete::new::<DbBookTranslation>(Expression::new(ValueIn::new(
      (DbBookTranslation::TABLE_NAME, "fktranslation"),
      &ids,
    )))
    .execute_transaction(self.transaction)
    .await?;

    Delete::new::<DbBook>(Expression::new(ValueIn::new((DbBook::TABLE_NAME, "id"), &ids)))
      .execute_transaction(self.transaction)
      .await?;
    Ok(())
  }
}

impl DefaultMutBookRepository<'_> {
  async fn insert_involved(&self, item: &CreatePartialBook, id: u32) -> Result<(), Box<dyn Error>> {
    if item.involved.is_empty() {
      return Ok(());
    }
    self.mut_book_involved_repository.add(id, &item.involved).await
  }

  async fn insert_genres(&self, item: &CreatePartialBook, id: u32) -> Result<(), Box<dyn Error>> {
    if item.genres.is_empty() {
      return Ok(());
    }
    self.mut_book_genre_repository.add(id, &item.genres).await
  }
  async fn insert_themes(&self, item: &CreatePartialBook, id: u32) -> Result<(), Box<dyn Error>> {
    if item.themes.is_empty() {
      return Ok(());
    }
    self.mut_book_theme_repository.add(id, &item.themes).await
  }
  async fn insert_characters(&self, item: &CreatePartialBook, id: u32) -> Result<(), Box<dyn Error>> {
    if item.characters.is_empty() {
      return Ok(());
    }
    self.mut_book_character_repository.add(id, &item.characters).await
  }
  async fn insert_book(&self, item: &CreatePartialBook) -> Result<i32, Box<dyn Error>> {
    let franchise = &item.franchise.map(|x| x as i32);
    let slug = item.slug.to_string();
    let book_id: i32 = Insert::new::<DbBook>(["published", "fkfranchise", "slug"])
      .values([&item.published, franchise, &slug])
      .returning_transaction("id", self.transaction)
      .await?;

    let rating_id: i32 = Insert::new::<DbRating>([])
      .values([])
      .returning_transaction("id", self.transaction)
      .await?;

    let (book_count,) = Select::new::<DbBook>()
      .count()
      .get_single(self.transaction.client())
      .await?
      .ok_or("DbBook count returned no columns")?;
    let book_count = book_count as i32;

    Insert::new::<DbBookStatistic>(["fkbook", "fkrating", "popularity", "rank"])
      .values([&book_id, &rating_id, &book_count, &book_count])
      .execute_transaction(self.transaction)
      .await?;

    Ok(book_id)
  }
  async fn insert_translation(&self, item: &CreatePartialBook, id: u32) -> Result<(), Box<dyn Error>> {
    let id = id as i32;
    let mapped: Vec<(&String, &Option<String>, i32, DbLanguage)> = item
      .translations
      .iter()
      .map(|x| {
        (
          &x.1.title,
          &x.1.description,
          x.1.cover.id as i32,
          DbLanguage::from(*x.0),
        )
      })
      .collect();
    let mut insert = Insert::new::<DbBookTranslation>(["title", "description", "fkcover", "fktranslation", "language"]);
    for (title, description, cover_id, language) in &mapped {
      insert.values_ref([*title, *description, cover_id, &id, language]);
    }
    insert.execute_transaction(self.transaction).await?;
    Ok(())
  }
}
