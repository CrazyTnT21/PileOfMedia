use std::collections::HashMap;
use std::error::Error;
use std::hash::Hash;
use std::sync::Arc;

use async_trait::async_trait;
use tokio_postgres::Client;

use domain::available_translations::AvailableTranslations;
use domain::entities::book::book_statistic::BookStatistic;
use domain::entities::book::book_translation::BookTranslation;
use domain::entities::book::Book;
use domain::entities::franchise::Franchise;
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
use crate::schemas::db_book::DbBook;
use crate::schemas::db_book_statistic::DbBookStatistic;
use crate::schemas::db_book_translation::DbBookTranslation;
use crate::schemas::db_rating::DbRating;
use crate::select::conditions::column_equal::ColumnEqual;
use crate::select::conditions::value_equal::ValueEqual;
use crate::select::conditions::value_ilike::ValueILike;
use crate::select::conditions::value_in::ValueIn;
use crate::select::expression::Expression;
use crate::select::Select;

pub struct DefaultBookRepository<'a> {
  client: &'a Client,
  image_repository: Arc<dyn ImageRepository + 'a>,
  franchise_repository: Arc<dyn FranchiseRepository + 'a>,
}

impl<'a> DefaultBookRepository<'a> {
  pub fn new(
    client: &'a Client,
    image_repository: Arc<dyn ImageRepository + 'a>,
    franchise_repository: Arc<dyn FranchiseRepository + 'a>,
  ) -> DefaultBookRepository<'a> {
    DefaultBookRepository {
      client,
      image_repository,
      franchise_repository,
    }
  }
}

#[async_trait]
impl BookRepository for DefaultBookRepository<'_> {
  async fn get(&self, languages: &[Language], pagination: Pagination) -> Result<ItemsTotal<Book>, Box<dyn Error>> {
    let total = Select::new::<DbBook>().query_count(self.client).await? as usize;

    let book_ids: Box<[u32]> = Select::new::<DbBook>()
      .column::<i32>(DbBook::TABLE_NAME, "id")
      .pagination(pagination)
      .query_destruct(self.client)
      .await?
      .into_iter()
      .map(|x| x as u32)
      .collect();

    let result = self.get_by_ids(&book_ids, languages).await?;
    Ok(ItemsTotal { items: result, total })
  }

  async fn get_by_id(&self, id: u32, languages: &[Language]) -> Result<Option<Book>, Box<dyn Error>> {
    let id = id as i32;
    let db_languages: Vec<DbLanguage> = languages.iter().map(|x| (*x).into()).collect();

    let book = Select::new::<DbBook>()
      .columns_table::<DbBook>()
      .distinct_on(DbBook::TABLE_NAME, "id")
      .inner_join::<DbBookTranslation>(
        None,
        Expression::new(ValueEqual::new((DbBook::TABLE_NAME, "id"), id)).and(book_id_equal_fk_translation()),
      )
      .get_single_destruct(self.client)
      .await?;
    let Some(item) = book else {
      return Ok(None);
    };
    let translations = Select::new::<DbBookTranslation>()
      .columns::<DbBookTranslation>(DbBookTranslation::TABLE_NAME)
      .where_expression(fk_translation_equal_id(item.id).and(in_languages(&db_languages)))
      .query_destruct(self.client)
      .await?;

    let image_ids = image_ids(&translations);
    let images = self.image_repository.get_by_ids(&image_ids).await?;

    let translations: Vec<(Language, BookTranslation)> = translations
      .into_iter()
      .map(|x| {
        let image = images.iter().find(|y| y.id as i32 == x.fk_cover).unwrap().clone();
        (Language::from(x.language), x.to_entity(image))
      })
      .collect();
    let mut available = self.available_languages(&[id]).await?;

    let franchise = match item.fk_franchise {
      None => None,
      Some(x) => Some(self.franchise_repository.get_by_id(x as u32, languages).await?.unwrap()),
    };
    let item = item.to_entity(
      AvailableTranslations {
        available_languages: available.remove(&id).unwrap(),
        translations: HashMap::from_iter(translations),
      },
      franchise,
    );
    Ok(Some(item))
  }

  async fn get_by_title(
    &self,
    title: &str,
    languages: &[Language],
    pagination: Pagination,
  ) -> Result<ItemsTotal<Book>, Box<dyn Error>> {
    let db_languages: Vec<DbLanguage> = languages.iter().map(|x| (*x).into()).collect();
    let title = format!("%{title}%");

    let total = Select::new::<DbBookTranslation>()
      .where_expression(book_translation_with_title(&title))
      .where_expression(in_languages(&db_languages))
      .query_count(self.client)
      .await? as usize;

    let book_ids: Box<[u32]> = Select::new::<DbBookTranslation>()
      .column::<i32>(DbBookTranslation::TABLE_NAME, "fktranslation")
      .distinct_on(DbBookTranslation::TABLE_NAME, "fktranslation")
      .where_expression(book_translation_with_title(&title))
      .where_expression(in_languages(&db_languages))
      .pagination(pagination)
      .query_destruct(self.client)
      .await?
      .into_iter()
      .map(|x| x as u32)
      .collect();

    let result = self.get_by_ids(&book_ids, languages).await?;
    Ok(ItemsTotal { items: result, total })
  }

  async fn get_by_ids(&self, ids: &[u32], languages: &[Language]) -> Result<Vec<Book>, Box<dyn Error>> {
    let ids = to_i32(ids);
    let db_languages: Vec<DbLanguage> = languages.iter().map(|x| (*x).into()).collect();

    let books = Select::new::<DbBook>()
      .columns_table::<DbBook>()
      .distinct_on(DbBook::TABLE_NAME, "id")
      .where_expression(Expression::new(ValueIn::new((DbBook::TABLE_NAME, "id"), &ids)))
      .query_destruct(self.client)
      .await?;

    let translations = Select::new::<DbBookTranslation>()
      .columns::<DbBookTranslation>(DbBookTranslation::TABLE_NAME)
      .where_expression(Expression::new(ValueIn::new(
        (DbBookTranslation::TABLE_NAME, "fktranslation"),
        &ids,
      )))
      .query_destruct(self.client)
      .await?;

    let image_ids: Vec<u32> = translations.iter().map(|x| x.fk_cover as u32).collect();
    let images = self.image_repository.get_by_ids(&image_ids).await?;
    let translations: Vec<(Language, i32, BookTranslation)> = book_translation_select(&ids, &db_languages)
      .query_destruct(self.client)
      .await?
      .into_iter()
      .map(|x| {
        let fk_cover = x.fk_cover;
        (
          x.language.into(),
          x.fk_translation,
          x.to_entity(images.iter().find(|y| y.id as i32 == fk_cover).unwrap().clone()),
        )
      })
      .collect();

    let franchise_ids = franchise_ids(&books);
    let franchises = self.franchise_repository.get_by_ids(&franchise_ids, languages).await?;

    let translations = map_translation(&books, translations);
    let available = self.available_languages(&ids).await?;
    let books = to_entities(books, available, translations, franchises);
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

  async fn get_by_slug(&self, slug: &Slug, languages: &[Language]) -> Result<Option<Book>, Box<dyn Error>> {
    let book_ids: Box<[u32]> = Select::new::<DbBook>()
      .column::<i32>(DbBook::TABLE_NAME, "id")
      .where_expression(book_slug_equal_slug(&slug.to_string()))
      .query_destruct(self.client)
      .await?
      .into_iter()
      .map(|x| x as u32)
      .collect();

    let mut result = self.get_by_ids(&book_ids, languages).await?;
    if result.is_empty() {
      return Ok(None);
    }
    Ok(Some(result.swap_remove(0)))
  }
}
impl DefaultBookRepository<'_> {
  async fn available_languages(&self, ids: &[i32]) -> Result<HashMap<i32, Vec<Language>>, Box<dyn Error>> {
    let available_translations = Select::new::<DbBookTranslation>()
      .column::<i32>(DbBookTranslation::TABLE_NAME, "fktranslation")
      .column::<DbLanguage>(DbBookTranslation::TABLE_NAME, "language")
      .where_expression(Expression::new(ValueIn::new(
        (DbBookTranslation::TABLE_NAME, "fktranslation"),
        ids,
      )))
      .query(self.client)
      .await?;
    let available_translations: Vec<(i32, Language)> = available_translations
      .into_iter()
      .map(|x| (x.0, Language::from(x.1)))
      .collect();
    let available = vec_tuple_to_map(available_translations);
    Ok(available)
  }
}

fn book_slug_equal_slug(slug: &str) -> Expression {
  Expression::value_equal(DbBook::TABLE_NAME, "slug", slug)
}
fn vec_tuple_to_map<K: Hash + Eq, V>(values: Vec<(K, V)>) -> HashMap<K, Vec<V>> {
  let mut map = HashMap::new();
  for (key, value) in values {
    match map.get_mut(&key) {
      None => {
        map.insert(key, vec![value]);
      }
      Some(v) => {
        v.push(value);
      }
    }
  }
  map
}

fn book_id_equal_fk_translation<'a>() -> Expression<'a> {
  Expression::column_equal(
    (DbBook::TABLE_NAME, "id"),
    (DbBookTranslation::TABLE_NAME, "fktranslation"),
  )
}
fn book_translation_with_title(title: &String) -> Expression {
  Expression::new(ValueILike::new((DbBookTranslation::TABLE_NAME, "title"), title))
}

fn in_languages(languages: &[DbLanguage]) -> Expression {
  Expression::new(ValueIn::new((DbBookTranslation::TABLE_NAME, "language"), languages))
}
fn fk_translation_equal_id<'a>(id: i32) -> Expression<'a> {
  Expression::value_equal(DbBookTranslation::TABLE_NAME, "fktranslation", id)
}

fn book_translation_select<'a>(
  book_ids: &'a [i32],
  db_languages: &'a [DbLanguage],
) -> Select<'a, (DbBookTranslation,)> {
  Select::new::<DbBookTranslation>()
    .columns::<DbBookTranslation>(DbBookTranslation::TABLE_NAME)
    .where_expression(Expression::new(ValueIn::new(
      (DbBookTranslation::TABLE_NAME, "fktranslation"),
      book_ids,
    )))
    .where_expression(in_languages(db_languages))
}
fn map_translation(
  books: &[DbBook],
  translations: Vec<(Language, i32, BookTranslation)>,
) -> HashMap<i32, Vec<(Language, BookTranslation)>> {
  let mut new_translations: HashMap<i32, Vec<(Language, BookTranslation)>> = HashMap::new();
  for book in books {
    new_translations.insert(book.id, vec![]);
  }
  for (language, id, translation) in translations {
    let result = new_translations.get_mut(&id).unwrap();
    result.push((language, translation));
  }
  new_translations
}
fn to_entities(
  books: Vec<DbBook>,
  mut available: HashMap<i32, Vec<Language>>,
  mut translations: HashMap<i32, Vec<(Language, BookTranslation)>>,
  franchises: Vec<Franchise>,
) -> Vec<Book> {
  books
    .into_iter()
    .map(|book| {
      let id = book.id;
      let franchise = book
        .fk_franchise
        .map(|x| franchises.iter().find(|y| y.id as i32 == x).unwrap().clone());
      book.to_entity(
        AvailableTranslations {
          available_languages: available.remove(&id).unwrap(),
          translations: HashMap::from_iter(translations.remove(&id).unwrap()),
        },
        franchise,
      )
    })
    .collect()
}

fn image_ids(items: &[DbBookTranslation]) -> Vec<u32> {
  let mut result = items.iter().map(|x| x.fk_cover as u32).collect::<Vec<u32>>();
  result.sort_unstable();
  result.dedup();
  result
}

fn franchise_ids(items: &[DbBook]) -> Vec<u32> {
  let mut result = items
    .iter()
    .filter_map(|x| x.fk_franchise.map(|x| x as u32))
    .collect::<Vec<u32>>();
  result.sort_unstable();
  result.dedup();
  result
}
