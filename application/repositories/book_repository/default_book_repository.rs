use std::error::Error;
use std::sync::Arc;

use async_trait::async_trait;
use tokio_postgres::Client;

use domain::entities::book::book_statistic::BookStatistic;
use domain::entities::book::Book;
use domain::enums::language::Language;
use domain::items_total::ItemsTotal;
use domain::pagination::Pagination;
use domain::slug::Slug;
use from_row::Table;
use repositories::book_repository::BookRepository;
use repositories::franchise_repository::FranchiseRepository;
use repositories::image_repository::ImageRepository;

use crate::convert_to_sql::to_i32;
use crate::enums::db_language::DbLanguage;
use crate::fallback_unwrap::{fallback_unwrap, fallback_unwrap_ref};
use crate::schemas::db_book::DbBook;
use crate::schemas::db_book_statistic::DbBookStatistic;
use crate::schemas::db_book_translation::DbBookTranslation;
use crate::schemas::db_rating::DbRating;
use crate::select::combined_tuple::CombinedType;
use crate::select::conditions::column_equal::ColumnEqual;
use crate::select::conditions::column_null::ColumnNull;
use crate::select::conditions::value_equal::ValueEqual;
use crate::select::conditions::value_ilike::ValueILike;
use crate::select::conditions::value_in::ValueIn;
use crate::select::expression::Expression;
use crate::select::Select;

pub struct DefaultBookRepository<'a> {
  client: &'a Client,
  default_language: DbLanguage,
  image_repository: Arc<dyn ImageRepository + 'a>,
  franchise_repository: Arc<dyn FranchiseRepository + 'a>,
}

impl<'a> DefaultBookRepository<'a> {
  pub fn new(
    client: &'a Client,
    default_language: Language,
    image_repository: Arc<dyn ImageRepository + 'a>,
    franchise_repository: Arc<dyn FranchiseRepository + 'a>,
  ) -> DefaultBookRepository<'a> {
    DefaultBookRepository {
      client,
      default_language: default_language.into(),
      image_repository,
      franchise_repository,
    }
  }

  async fn books_from_tuple(&self, items: Vec<BookColumns>, language: Language) -> Result<Vec<Book>, Box<dyn Error>> {
    if items.is_empty() {
      return Ok(vec![]);
    }

    let image_ids = image_ids(&items);
    let franchise_ids = franchise_ids(&items);
    let images = self.image_repository.get_by_ids(&image_ids).await?;

    //TODO
    let franchises = self
      .franchise_repository
      .get_by_ids(&franchise_ids, &[language])
      .await?;

    items
      .into_iter()
      .map(|item| {
        let book_translation = fallback_unwrap(item.1, item.2);
        let franchise = franchises
          .iter()
          .find(|y| match item.0.fk_franchise {
            None => false,
            Some(id) => id as u32 == y.id,
          })
          .cloned();
        let image = images
          .iter()
          .find(|y| y.id == book_translation.fk_cover as u32)
          .unwrap()
          .clone();
        Ok(item.0.to_entity(book_translation, image, franchise))
      })
      .collect()
  }
  async fn book_from_tuple(&self, item: BookColumns, language: Language) -> Result<Book, Box<dyn Error>> {
    let book_translation = fallback_unwrap(item.1, item.2);
    let image = self
      .image_repository
      .get_by_id(book_translation.fk_cover as u32)
      .await?
      .unwrap();

    //TODO
    let franchise = match item.0.fk_franchise {
      None => None,
      Some(value) => self.franchise_repository.get_by_id(value as u32, &[language]).await?,
    };
    Ok(item.0.to_entity(book_translation, image, franchise))
  }
}

fn image_ids(items: &[BookColumns]) -> Vec<u32> {
  let mut result = items
    .iter()
    .map(|x| fallback_unwrap_ref(x.1.as_ref(), x.2.as_ref()).fk_cover as u32)
    .collect::<Vec<u32>>();
  result.sort_unstable();
  result.dedup();
  result
}

fn franchise_ids(items: &[BookColumns]) -> Vec<u32> {
  let mut result = items
    .iter()
    .filter_map(|x| x.0.fk_franchise.map(|x| x as u32))
    .collect::<Vec<u32>>();
  result.sort_unstable();
  result.dedup();
  result
}

#[async_trait]
impl BookRepository for DefaultBookRepository<'_> {
  async fn get(&self, language: Language, pagination: Pagination) -> Result<ItemsTotal<Book>, Box<dyn Error>> {
    let db_language = DbLanguage::from(language);

    let total = Select::new::<DbBook>()
      .transform(|x| book_joins(x, &db_language, &self.default_language))
      .query_count(self.client)
      .await? as usize;

    let books = book_select(&db_language, &self.default_language)
      .pagination(pagination)
      .query(self.client)
      .await?;

    let books = self.books_from_tuple(books, language).await?;
    Ok(ItemsTotal { items: books, total })
  }

  async fn get_by_id(&self, id: u32, language: Language) -> Result<Option<Book>, Box<dyn Error>> {
    let id = id as i32;
    let db_language = DbLanguage::from(language);

    let select = book_select(&db_language, &self.default_language)
      .where_expression(Expression::new(ValueEqual::new(("book", "id"), id)));

    let Some(value) = select.get_single(self.client).await? else {
      return Ok(None);
    };
    Ok(Some(self.book_from_tuple(value, language).await?))
  }

  async fn get_by_title(
    &self,
    title: &str,
    language: Language,
    pagination: Pagination,
  ) -> Result<ItemsTotal<Book>, Box<dyn Error>> {
    let title = format!("%{title}%");
    let db_language = DbLanguage::from(language);

    let total = Select::new::<DbBook>()
      .transform(|x| book_joins(x, &db_language, &self.default_language))
      .where_expression(
        Expression::new(ValueILike::new(("book_translation", "title"), &title)).or(Expression::new(ValueILike::new(
          ("book_translation_fallback", "title"),
          &title,
        ))),
      )
      .query_count(self.client)
      .await? as usize;

    let books = book_select(&db_language, &self.default_language)
      .where_expression(
        Expression::new(ValueILike::new(("book_translation", "title"), &title)).or(Expression::new(ValueILike::new(
          ("book_translation_fallback", "title"),
          &title,
        ))),
      )
      .pagination(pagination)
      .query(self.client)
      .await?;
    let books = self.books_from_tuple(books, language).await?;
    Ok(ItemsTotal { items: books, total })
  }

  async fn get_by_ids(&self, ids: &[u32], language: Language) -> Result<Vec<Book>, Box<dyn Error>> {
    let db_language = DbLanguage::from(language);
    let ids = to_i32(ids);

    let books = book_select(&db_language, &self.default_language)
      .where_expression(Expression::new(ValueIn::new((DbBook::TABLE_NAME, "id"), &ids)))
      .query(self.client)
      .await?;

    let books = self.books_from_tuple(books, language).await?;

    Ok(books)
  }

  async fn filter_existing(&self, book_ids: &[u32]) -> Result<Vec<u32>, Box<dyn Error>> {
    let books = to_i32(book_ids);

    let filtered = Select::new::<DbBook>()
      .column::<i32>(DbBook::TABLE_NAME, "id")
      .where_expression(Expression::new(ValueIn::new((DbBook::TABLE_NAME, "id"), &books)))
      .query(self.client)
      .await?
      .into_iter()
      .map(|x| x.0 as u32)
      .collect();
    Ok(filtered)
  }

  async fn get_statistics(&self, book_ids: &[u32]) -> Result<Vec<BookStatistic>, Box<dyn Error>> {
    let ids = to_i32(book_ids);

    let statistics = Select::new::<DbBookStatistic>()
      .columns::<DbBookStatistic>(DbBookStatistic::TABLE_NAME)
      .columns::<DbRating>(DbRating::TABLE_NAME)
      .inner_join::<DbRating>(
        None,
        Expression::new(ColumnEqual::new(
          (DbRating::TABLE_NAME, "id"),
          (DbBookStatistic::TABLE_NAME, "fkrating"),
        )),
      )
      .where_expression(Expression::new(ValueIn::new(
        (DbBookStatistic::TABLE_NAME, "fkbook"),
        &ids,
      )))
      .query(self.client)
      .await?
      .into_iter()
      .map(|(statistic, rating)| statistic.to_entity(rating.to_entity()))
      .collect();

    Ok(statistics)
  }

  async fn get_by_slug(&self, slug: &Slug, language: Language) -> Result<Option<Book>, Box<dyn Error>> {
    let db_language = DbLanguage::from(language);
    let slug = slug.to_string();
    let select = book_select(&db_language, &self.default_language).where_expression(Expression::value_equal(
      DbBook::TABLE_NAME,
      "slug",
      slug,
    ));

    let Some(value) = select.get_single(self.client).await? else {
      return Ok(None);
    };
    Ok(Some(self.book_from_tuple(value, language).await?))
  }
}

fn book_select<'a>(language: &'a DbLanguage, fallback_language: &'a DbLanguage) -> Select<'a, BookColumns> {
  book_select_columns().transform(|x| book_joins(x, language, fallback_language))
}

fn book_joins<'a, T: from_row::FromRow<DbType = T> + CombinedType>(
  select: Select<'a, T>,
  language: &'a DbLanguage,
  fallback_language: &'a DbLanguage,
) -> Select<'a, T> {
  select
    .left_join::<DbBookTranslation>(
      Some("book_translation"),
      Expression::new(ColumnEqual::new(("book_translation", "fktranslation"), ("book", "id"))).and(Expression::new(
        ValueEqual::new(("book_translation", "language"), language),
      )),
    )
    .left_join::<DbBookTranslation>(
      Some("book_translation_fallback"),
      Expression::new(ColumnEqual::new(
        ("book", "id"),
        ("book_translation_fallback", "fktranslation"),
      ))
      .and(Expression::new(ColumnNull::new(("book_translation", "fktranslation"))))
      .and(Expression::new(ValueEqual::new(
        ("book_translation_fallback", "language"),
        fallback_language,
      ))),
    )
}

fn book_select_columns<'a>() -> Select<'a, BookColumns> {
  Select::new::<DbBook>()
    .columns::<DbBook>("book")
    .columns::<Option<DbBookTranslation>>("book_translation")
    .columns::<Option<DbBookTranslation>>("book_translation_fallback")
}

type BookColumns = (DbBook, Option<DbBookTranslation>, Option<DbBookTranslation>);
