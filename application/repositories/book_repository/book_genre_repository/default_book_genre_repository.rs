use std::error::Error;
use std::sync::Arc;

use async_trait::async_trait;
use tokio_postgres::Client;

use crate::convert_to_sql::to_i32;
use domain::entities::genre::Genre;
use domain::enums::language::Language;
use domain::items_total::ItemsTotal;
use domain::pagination::Pagination;
use from_row::Table;
use repositories::book_repository::book_genre_repository::BookGenreRepository;
use repositories::book_repository::BookRepository;
use repositories::genre_repository::GenreRepository;

use crate::enums::db_language::DbLanguage;
use crate::schemas::db_book_genre::DbBookGenre;
use crate::select::conditions::value_equal::ValueEqual;
use crate::select::conditions::value_in::ValueIn;
use crate::select::expression::Expression;
use crate::select::Select;

pub struct DefaultBookGenreRepository<'a> {
  client: &'a Client,
  default_language: DbLanguage,
  book_repository: Arc<dyn BookRepository + 'a>,
  genre_repository: Arc<dyn GenreRepository + 'a>,
}

impl<'a> DefaultBookGenreRepository<'a> {
  pub fn new(
    client: &'a Client,
    default_language: Language,
    book_repository: Arc<dyn BookRepository + 'a>,
    genre_repository: Arc<dyn GenreRepository + 'a>,
  ) -> DefaultBookGenreRepository<'a> {
    DefaultBookGenreRepository {
      client,
      default_language: default_language.into(),
      book_repository,
      genre_repository,
    }
  }
}

#[async_trait]
impl<'a> BookGenreRepository for DefaultBookGenreRepository<'a> {
  async fn get(
    &self,
    book_id: u32,
    language: Language,
    pagination: Pagination,
  ) -> Result<ItemsTotal<Genre>, Box<dyn Error>> {
    let book_id = book_id as i32;

    let total = Select::new::<DbBookGenre>()
      .where_expression(Expression::new(ValueEqual::new(
        (DbBookGenre::TABLE_NAME, "fkbook"),
        book_id,
      )))
      .count()
      .get_single(self.client)
      .await?
      .expect("Count should return one row");
    let total = total.0 as usize;

    let genre_ids: Vec<u32> = Select::new::<DbBookGenre>()
      .column::<i32>(DbBookGenre::TABLE_NAME, "fkgenre")
      .where_expression(Expression::new(ValueEqual::new(
        (DbBookGenre::TABLE_NAME, "fkbook"),
        book_id,
      )))
      .pagination(pagination)
      .query(self.client)
      .await?
      .into_iter()
      .map(|x| x.0 as u32)
      .collect();

    let items = match genre_ids.is_empty() {
      true => vec![],
      false => self.genre_repository.get_by_ids(&genre_ids, language).await?,
    };
    Ok(ItemsTotal { items, total })
  }

  async fn filter_existing(&self, book_id: u32, genres: &[u32]) -> Result<Vec<u32>, Box<dyn Error>> {
    let book_id = book_id as i32;
    let genres = to_i32(genres);

    let filtered = Select::new::<DbBookGenre>()
      .column::<i32>(DbBookGenre::TABLE_NAME, "fkgenre")
      .where_expression(Expression::new(ValueIn::new(
        (DbBookGenre::TABLE_NAME, "fkgenre"),
        &genres,
      )))
      .where_expression(Expression::column_equal(DbBookGenre::TABLE_NAME, "fkbook", book_id))
      .query(self.client)
      .await?
      .into_iter()
      .map(|x| x.0 as u32)
      .collect();
    Ok(filtered)
  }
}
