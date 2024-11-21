use std::error::Error;
use std::sync::Arc;

use async_trait::async_trait;
use tokio_postgres::Transaction;

use domain::entities::genre::create_partial_genre::CreatePartialGenre;
use domain::entities::genre::Genre;
use domain::enums::language::Language;
use from_row::Table;
use repositories::genre_repository::mut_genre_repository::MutGenreRepository;
use repositories::genre_repository::GenreRepository;

use crate::convert_to_sql::to_i32;
use crate::delete::Delete;
use crate::enums::db_language::DbLanguage;
use crate::insert::Insert;
use crate::schemas::db_genre::DbGenre;
use crate::schemas::db_genre_translation::DbGenreTranslation;
use crate::select::conditions::value_in::ValueIn;
use crate::select::expression::Expression;

pub struct DefaultMutGenreRepository<'a> {
  transaction: &'a Transaction<'a>,
  default_language: Language,
  genre_repository: Arc<dyn GenreRepository + 'a>,
}

impl<'a> DefaultMutGenreRepository<'a> {
  pub fn new(
    transaction: &'a Transaction<'a>,
    default_language: Language,
    genre_repository: Arc<dyn GenreRepository + 'a>,
  ) -> DefaultMutGenreRepository<'a> {
    DefaultMutGenreRepository {
      transaction,
      default_language,
      genre_repository,
    }
  }
}

#[async_trait]
impl MutGenreRepository for DefaultMutGenreRepository<'_> {
  async fn create(&self, item: CreatePartialGenre) -> Result<Genre, Box<dyn Error>> {
    let id = self.insert_genre(&item).await? as u32;
    self.insert_translation(&item, id).await?;

    let genre = self
      .genre_repository
      .get_by_id(id, self.default_language)
      .await?
      .expect("Genre was just created");
    Ok(genre)
  }

  async fn delete(&self, ids: &[u32]) -> Result<(), Box<dyn Error>> {
    let ids = to_i32(ids);

    Delete::new::<DbGenreTranslation>(Expression::new(ValueIn::new(
      (DbGenreTranslation::TABLE_NAME, "fktranslation"),
      &ids,
    )))
    .execute_transaction(self.transaction)
    .await?;

    Delete::new::<DbGenre>(Expression::new(ValueIn::new((DbGenre::TABLE_NAME, "id"), &ids)))
      .execute_transaction(self.transaction)
      .await?;
    Ok(())
  }
}

impl DefaultMutGenreRepository<'_> {
  async fn insert_genre(&self, _item: &CreatePartialGenre) -> Result<i32, Box<dyn Error>> {
    let id = Insert::new::<DbGenre>([])
      .returning_transaction("id", self.transaction)
      .await?;
    Ok(id)
  }
  async fn insert_translation(&self, item: &CreatePartialGenre, id: u32) -> Result<(), Box<dyn Error>> {
    let id = id as i32;
    let translations: Vec<(&String, DbLanguage)> = item
      .translations
      .iter()
      .map(|x| (&x.1.name, DbLanguage::from(*x.0)))
      .collect();

    let mut insert = Insert::new::<DbGenreTranslation>(["name", "fktranslation", "language"]);
    for (title, language) in &translations {
      insert.values_ref([*title, &id, language]);
    }
    insert.execute_transaction(self.transaction).await?;
    Ok(())
  }
}
