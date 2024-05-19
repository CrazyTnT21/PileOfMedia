use std::error::Error;

use async_trait::async_trait;
use bb8_postgres::bb8::PooledConnection;
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::NoTls;

use domain::entities::book::book::Book;
use domain::enums::language::Language;
use domain::items_total::ItemsTotal;
use domain::pagination::Pagination;
use repositories::book_repository::{BookRepository};

use crate::enums::db_language::DbLanguage;
use crate::schemas::db_book::DbBook;
use crate::schemas::db_book_translation::DbBookTranslation;
use crate::schemas::db_franchise::DbFranchise;
use crate::schemas::db_image::DbImage;
use crate::select::comparison::Comparison::{Equal, ILike, IsNull};
use crate::select::condition::Condition::{Column, Value};
use crate::select::expression::Expression;
use crate::select::select::Select;

pub struct DefaultBookRepository {
  pool: PooledConnection<'static, PostgresConnectionManager<NoTls>>,
  default_language: Language,
}

impl DefaultBookRepository {
  pub fn new(pool: PooledConnection<'static, PostgresConnectionManager<NoTls>>, default_language: Language) -> DefaultBookRepository {
    DefaultBookRepository { pool, default_language }
  }
}

#[async_trait]
impl BookRepository for DefaultBookRepository {
  async fn get(&self, language: Language, pagination: Pagination) -> Result<ItemsTotal<Book>, Box<dyn Error>> {
    let language = DbLanguage::from(language);
    let fallback_language = DbLanguage::from(self.default_language);

    let select = book_select(&language, &fallback_language);

    let total = select.count(&self.pool).await? as usize;
    let select = select.pagination(pagination);
    let books = select
      .query(&self.pool)
      .await?
      .into_iter()
      .map(book_from_tuple)
      .collect();

    Ok(ItemsTotal { items: books, total })
  }

  async fn get_by_id(&self, id: u32, language: Language) -> Result<Option<Book>, Box<dyn Error>> {
    let id = id as i32;
    let language = DbLanguage::from(language);
    let fallback_language = DbLanguage::from(self.default_language);

    let select = book_select(&language, &fallback_language)
      .where_expression(Expression::new(Value(("book", "id"), Equal(&id))));

    Ok(select.get_single(&self.pool)
      .await?
      .and_then(|x| Some(book_from_tuple(x))))
  }

  async fn get_by_title(&self, title: &str, language: Language, pagination: Pagination) -> Result<ItemsTotal<Book>, Box<dyn Error>> {
    let title = format!("%{title}%");
    let language = DbLanguage::from(language);
    let fallback_language = DbLanguage::from(self.default_language);

    let select = book_select(&language, &fallback_language)
      .where_expression(Expression::new(Value(("book_translation", "title"), ILike(&title)))
        .or(Expression::new(Value(("book_translation_fallback", "title"), ILike(&title)))));

    let total = select.count(&self.pool).await? as usize;

    let select = select.pagination(pagination);

    let books = select
      .query(&self.pool)
      .await?
      .into_iter()
      .map(book_from_tuple)
      .collect();
    Ok(ItemsTotal { items: books, total })
  }
}

fn book_select<'a>(language: &'a DbLanguage, fallback_language: &'a DbLanguage) -> Select<'a, (DbBook, Option<DbBookTranslation>, Option<DbBookTranslation>, Option<DbImage>, Option<DbImage>, Option<DbFranchise>)> {
  book_select_columns()
    .left_join("booktranslation", Some("book_translation"),
               Expression::new(Column(("book_translation", "fktranslation"), ("book", "id")))
                 .and(Expression::new(Value(("book_translation", "language"), Equal(language)))))
    .left_join("booktranslation", Some("book_translation_fallback"),
               Expression::new(Column(("book", "id"), ("book_translation_fallback", "fktranslation")))
                 .and(Expression::new(Value(("book_translation", "fktranslation"), IsNull)))
                 .and(Expression::new(Value(("book_translation_fallback", "language"), Equal(fallback_language)))))
    .left_join("image", Some("cover"), Expression::new(Column(("cover", "id"), ("book_translation", "fkcover"))))
    .left_join("image", Some("cover_fallback"), Expression::new(Column(("cover_fallback", "id"), ("book_translation_fallback", "fkcover"))))
    .left_join("franchise", None, Expression::new(Column(("book", "fkfranchise"), ("franchise", "id"))))
}
fn book_from_tuple(query_result: (DbBook, Option<DbBookTranslation>, Option<DbBookTranslation>, Option<DbImage>, Option<DbImage>, Option<DbFranchise>)) -> Book {
  query_result.0.to_entity(
    query_result.1.unwrap_or_else(|| query_result.2.expect("Fallback for book translation should exist")),
    query_result.3.unwrap_or_else(|| query_result.4.expect("Fallback for book cover translation should exist")),
    query_result.5)
}

fn book_select_columns<'a>() -> Select<'a, (DbBook, Option<DbBookTranslation>, Option<DbBookTranslation>, Option<DbImage>, Option<DbImage>, Option<DbFranchise>)> {
  Select::new("book")
    .columns::<DbBook>("book")
    .columns::<Option<DbBookTranslation>>("book_translation")
    .columns::<Option<DbBookTranslation>>("book_translation_fallback")
    .columns::<Option<DbImage>>("cover")
    .columns::<Option<DbImage>>("cover_fallback")
    .columns::<Option<DbFranchise>>("franchise")
}
