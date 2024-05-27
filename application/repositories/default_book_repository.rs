use std::error::Error;

use async_trait::async_trait;

use domain::entities::book::Book;
use domain::enums::language::Language;
use domain::items_total::ItemsTotal;
use domain::pagination::Pagination;
use repositories::book_repository::BookRepository;
use repositories::image_repository::ImageRepository;

use crate::enums::db_language::DbLanguage;
use crate::fallback_unwrap::{fallback_unwrap, fallback_unwrap_ref};
use crate::Pooled;
use crate::schemas::db_book::DbBook;
use crate::schemas::db_book_translation::DbBookTranslation;
use crate::schemas::db_franchise::DbFranchise;
use crate::schemas::db_image::DbImage;
use crate::select::comparison::Comparison::{Equal, ILike, IsNull};
use crate::select::condition::Condition::{Column, Value};
use crate::select::expression::Expression;
use crate::select::Select;

pub struct DefaultBookRepository<'a> {
  pool: &'a Pooled<'a>,
  default_language: Language,
  image_repository: &'a dyn ImageRepository,
}

impl<'a> DefaultBookRepository<'a> {
  pub fn new(pool: &'a Pooled, default_language: Language, image_repository: &'a dyn ImageRepository) -> DefaultBookRepository<'a> {
    DefaultBookRepository { pool, default_language, image_repository }
  }

  async fn books_from_tuple(&self, items: Vec<BookColumns>) -> Result<Vec<Book>, Box<dyn Error>> {
    if items.is_empty() {
      return Ok(vec![]);
    }

    let image_ids = image_ids(&items);
    let mut images = self.image_repository.get_by_ids(&image_ids).await?;

    items
      .into_iter()
      .map(|item| {
        let book_translation = fallback_unwrap(item.1, item.2);
        let franchise = item.5.map(|x| x.to_entity());
        let image_index = images.iter().position(|y| y.id == book_translation.fk_cover).unwrap();
        let image = images.swap_remove(image_index);

        Ok(item.0.to_entity(book_translation, image, franchise))
      })
      .collect()
  }
  async fn book_from_tuple(&self, item: BookColumns) -> Result<Book, Box<dyn Error>> {
    let image_id = fallback_unwrap_ref(item.3.as_ref(), item.4.as_ref()).id as u32;
    let image = self.image_repository.get_by_id(image_id).await?.unwrap();
    let book_translation = fallback_unwrap(item.1, item.2);
    let franchise = item.5.map(|x| x.to_entity());
    Ok(item.0.to_entity(book_translation, image, franchise))
  }
}

fn image_ids(items: &[BookColumns]) -> Vec<i32> {
  items
    .iter()
    .map(|x| fallback_unwrap_ref(x.3.as_ref(), x.4.as_ref()).id)
    .collect::<Vec<i32>>()
}

#[async_trait]
impl BookRepository for DefaultBookRepository<'_> {
  async fn get(&self, language: Language, pagination: Pagination) -> Result<ItemsTotal<Book>, Box<dyn Error>> {
    let language = DbLanguage::from(language);
    let fallback_language = DbLanguage::from(self.default_language);

    let select = book_select(&language, &fallback_language);

    let total = select.count(self.pool).await? as usize;
    let select = select.pagination(pagination);
    let books = select
      .query(self.pool)
      .await?;

    let books = self.books_from_tuple(books).await?;
    Ok(ItemsTotal { items: books, total })
  }

  async fn get_by_id(&self, id: u32, language: Language) -> Result<Option<Book>, Box<dyn Error>> {
    let id = id as i32;
    let language = DbLanguage::from(language);
    let fallback_language = DbLanguage::from(self.default_language);

    let select = book_select(&language, &fallback_language)
      .where_expression(Expression::new(Value(("book", "id"), Equal(&id))));

    let Some(value) = select.get_single(self.pool).await? else {
      return Ok(None);
    };
    Ok(Some(self.book_from_tuple(value).await?))
  }

  async fn get_by_title(&self, title: &str, language: Language, pagination: Pagination) -> Result<ItemsTotal<Book>, Box<dyn Error>> {
    let title = format!("%{title}%");
    let language = DbLanguage::from(language);
    let fallback_language = DbLanguage::from(self.default_language);

    let select = book_select(&language, &fallback_language)
      .where_expression(Expression::new(Value(("book_translation", "title"), ILike(&title)))
        .or(Expression::new(Value(("book_translation_fallback", "title"), ILike(&title)))));

    let total = select.count(self.pool).await? as usize;

    let select = select.pagination(pagination);

    let books = select
      .query(self.pool)
      .await?;
    let books = self.books_from_tuple(books).await?;
    Ok(ItemsTotal { items: books, total })
  }
}

fn book_select<'a>(language: &'a DbLanguage, fallback_language: &'a DbLanguage) -> Select<'a, BookColumns> {
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

fn book_select_columns<'a>() -> Select<'a, BookColumns> {
  Select::new("book")
    .columns::<DbBook>("book")
    .columns::<Option<DbBookTranslation>>("book_translation")
    .columns::<Option<DbBookTranslation>>("book_translation_fallback")
    .columns::<Option<DbImage>>("cover")
    .columns::<Option<DbImage>>("cover_fallback")
    .columns::<Option<DbFranchise>>("franchise")
}
type BookColumns = (DbBook, Option<DbBookTranslation>, Option<DbBookTranslation>, Option<DbImage>, Option<DbImage>, Option<DbFranchise>);
