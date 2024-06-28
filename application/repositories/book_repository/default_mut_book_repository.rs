use std::error::Error;
use std::sync::Arc;

use async_trait::async_trait;
use tokio_postgres::Transaction;

use domain::entities::book::Book;
use domain::entities::book::create_partial_book::CreatePartialBook;
use domain::enums::language::Language;
use from_row::Table;
use repositories::book_repository::book_character_repository::mut_book_character_repository::MutBookCharacterRepository;
use repositories::book_repository::book_genre_repository::mut_book_genre_repository::MutBookGenreRepository;
use repositories::book_repository::book_involved_repository::mut_book_involved_repository::MutBookInvolvedRepository;
use repositories::book_repository::book_theme_repository::mut_book_theme_repository::MutBookThemeRepository;
use repositories::book_repository::BookRepository;
use repositories::book_repository::mut_book_repository::MutBookRepository;

use crate::convert_to_sql::to_i32;
use crate::delete::Delete;
use crate::enums::db_language::DbLanguage;
use crate::insert::Insert;
use crate::schemas::db_book::DbBook;
use crate::schemas::db_book_translation::DbBookTranslation;
use crate::select::conditions::value_in::ValueIn;
use crate::select::expression::Expression;

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
  pub fn new(transaction: &'a Transaction<'a>,
             default_language: Language,
             mut_book_genre_repository: Arc<dyn MutBookGenreRepository + 'a>,
             mut_book_character_repository: Arc<dyn MutBookCharacterRepository + 'a>,
             mut_book_theme_repository: Arc<dyn MutBookThemeRepository + 'a>,
             mut_book_involved_repository: Arc<dyn MutBookInvolvedRepository + 'a>,
             book_repository: Arc<dyn BookRepository + 'a>, ) -> DefaultMutBookRepository<'a> {
    DefaultMutBookRepository {
      transaction,
      mut_book_genre_repository,
      mut_book_character_repository,
      mut_book_theme_repository,
      mut_book_involved_repository,
      default_language,
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

    let book = self.book_repository
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

    Delete::new::<DbBookTranslation>(Expression::new(ValueIn::new((DbBookTranslation::TABLE_NAME, "fktranslation"), &ids)))
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
    let chapters = &item.chapters.map(|x| x as i16);
    let pages = &item.pages.map(|x| x as i16);
    let words = &item.words.map(|x| x as i32);
    let franchise = &item.franchise.map(|x| x as i32);
    let id = Insert::new::<DbBook>(["chapters", "pages", "words", "published", "fkfranchise"])
      .values([chapters, pages, words, &item.published, franchise])
      .returning_transaction("id", self.transaction).await?;
    Ok(id)
  }
  async fn insert_translation(&self, item: &CreatePartialBook, id: u32) -> Result<(), Box<dyn Error>> {
    let id = id as i32;
    let mapped: Vec<(&String, &Option<String>, i32, DbLanguage)> = item.translations
      .iter()
      .map(|x| (&x.1.title, &x.1.description, x.1.cover.id as i32, DbLanguage::from(*x.0)))
      .collect();
    let mut insert = Insert::new::<DbBookTranslation>(["title", "description", "fkcover", "fktranslation", "language"]);
    for (title, description, cover_id, language) in &mapped {
      insert.values_ref([*title, *description, cover_id, &id, language]);
    }
    insert.execute_transaction(self.transaction).await?;
    Ok(())
  }
}
