use std::collections::HashMap;
use std::error::Error;
use std::sync::Arc;

use async_trait::async_trait;
use tokio_postgres::Client;

use domain::available_translations::AvailableTranslations;
use domain::entities::book::book_character::BookCharacter;
use domain::entities::book::book_involved::BookInvolved;
use domain::entities::book::book_statistic::BookStatistic;
use domain::entities::book::book_translation::BookTranslation;
use domain::entities::book::Book;
use domain::entities::franchise::Franchise;
use domain::entities::genre::Genre;
use domain::entities::theme::Theme;
use domain::enums::language::Language;
use domain::items_total::ItemsTotal;
use domain::pagination::Pagination;
use domain::slug::Slug;
use domain::vec_tuple_to_map::vec_tuple_to_map;
use from_row::Table;
use repositories::book_repository::book_character_repository::BookCharacterRepository;
use repositories::book_repository::book_genre_repository::BookGenreRepository;
use repositories::book_repository::book_involved_repository::BookInvolvedRepository;
use repositories::book_repository::book_theme_repository::BookThemeRepository;
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
  book_genre_repository: Arc<dyn BookGenreRepository + 'a>,
  book_theme_repository: Arc<dyn BookThemeRepository + 'a>,
  book_involved_repository: Arc<dyn BookInvolvedRepository + 'a>,
  book_character_repository: Arc<dyn BookCharacterRepository + 'a>,
}

impl<'a> DefaultBookRepository<'a> {
  pub fn new(
    client: &'a Client,
    image_repository: Arc<dyn ImageRepository + 'a>,
    franchise_repository: Arc<dyn FranchiseRepository + 'a>,
    book_genre_repository: Arc<dyn BookGenreRepository + 'a>,
    book_theme_repository: Arc<dyn BookThemeRepository + 'a>,
    book_involved_repository: Arc<dyn BookInvolvedRepository + 'a>,
    book_character_repository: Arc<dyn BookCharacterRepository + 'a>,
  ) -> DefaultBookRepository<'a> {
    DefaultBookRepository {
      client,
      image_repository,
      franchise_repository,
      book_genre_repository,
      book_theme_repository,
      book_involved_repository,
      book_character_repository,
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
    let db_id = id as i32;
    let db_languages: Vec<DbLanguage> = languages.iter().map(|x| (*x).into()).collect();

    let book = Select::new::<DbBook>()
      .columns_table::<DbBook>()
      .distinct_on(DbBook::TABLE_NAME, "id")
      .inner_join::<DbBookTranslation>(
        None,
        Expression::new(ValueEqual::new((DbBook::TABLE_NAME, "id"), db_id)).and(book_id_equal_fk_translation()),
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
    let mut available = self.available_languages(&[db_id]).await?;

    let franchise = match item.fk_franchise {
      None => None,
      Some(x) => Some(self.franchise_repository.get_by_id(x as u32, languages).await?.unwrap()),
    };

    //TODO futures join!
    let statistic = self.get_statistics(&[id]).await?.remove(&id).unwrap();
    let genres = self.book_genre_repository.get_by_id(id, languages).await?;
    let themes = self.book_theme_repository.get_by_id(id, languages).await?;
    let involved = self.book_involved_repository.get_by_id(id, languages).await?;
    let characters = self.book_character_repository.get_by_id(id, languages).await?;
    let item = item.to_entity(
      AvailableTranslations {
        available_languages: available.remove(&db_id).unwrap(),
        translations: HashMap::from_iter(translations),
      },
      franchise,
      genres,
      themes,
      involved,
      characters,
      statistic,
    );
    Ok(Some(item))
  }

  async fn get_by_title(
    &self,
    title: &str,
    languages: &[Language],
    pagination: Pagination,
  ) -> Result<ItemsTotal<Book>, Box<dyn Error>> {
    let title = format!("%{title}%");

    let total = Select::new::<DbBookTranslation>()
      .where_expression(book_translation_with_title(&title))
      .query_count(self.client)
      .await? as usize;

    let book_ids: Box<[u32]> = Select::new::<DbBookTranslation>()
      .column::<i32>(DbBookTranslation::TABLE_NAME, "fktranslation")
      .distinct_on(DbBookTranslation::TABLE_NAME, "fktranslation")
      .where_expression(book_translation_with_title(&title))
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
    let db_ids = to_i32(ids);
    let db_languages: Vec<DbLanguage> = languages.iter().map(|x| (*x).into()).collect();

    let books = Select::new::<DbBook>()
      .columns_table::<DbBook>()
      .distinct_on(DbBook::TABLE_NAME, "id")
      .where_expression(Expression::new(ValueIn::new((DbBook::TABLE_NAME, "id"), &db_ids)))
      .query_destruct(self.client)
      .await?;
    let ids: Vec<u32> = books.iter().map(|x| x.id as u32).collect();
    let db_ids: Vec<i32> = books.iter().map(|x| x.id).collect();

    let mut translations = Select::new::<DbBookTranslation>()
      .columns::<DbBookTranslation>(DbBookTranslation::TABLE_NAME)
      .where_expression(Expression::new(ValueIn::new(
        (DbBookTranslation::TABLE_NAME, "fktranslation"),
        &db_ids,
      )))
      .where_expression(in_languages(&db_languages))
      .query_destruct(self.client)
      .await?;

    let no_translations: Vec<i32> = no_translation_ids(&books, &translations);
    let mut extra_translations = Select::new::<DbBookTranslation>()
      .distinct_on(DbBookTranslation::TABLE_NAME, "fktranslation")
      .columns::<DbBookTranslation>(DbBookTranslation::TABLE_NAME)
      .where_expression(fk_translation_in_ids(&no_translations))
      .query_destruct(self.client)
      .await?;

    translations.append(&mut extra_translations);

    let image_ids: Vec<u32> = translations.iter().map(|x| x.fk_cover as u32).collect();
    let images = self.image_repository.get_by_ids(&image_ids).await?;
    let translations: Vec<(Language, i32, BookTranslation)> = translations
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
    let available = self.available_languages(&db_ids).await?;

    let statistics = self.get_statistics(&ids).await?;
    let genres = self.book_genre_repository.get_by_ids(&ids, languages).await?;
    let themes = self.book_theme_repository.get_by_ids(&ids, languages).await?;
    let involved = self.book_involved_repository.get_by_ids(&ids, languages).await?;
    let characters = self.book_character_repository.get_by_ids(&ids, languages).await?;

    let books = to_entities(
      books,
      available,
      translations,
      franchises,
      genres,
      themes,
      involved,
      characters,
      statistics,
    );
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

  async fn get_statistics(&self, book_ids: &[u32]) -> Result<HashMap<u32, BookStatistic>, Box<dyn Error>> {
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
      .map(|(statistic, rating)| (statistic.fk_book as u32, statistic.to_entity(rating.to_entity())))
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
  genres: HashMap<u32, Vec<Genre>>,
  themes: HashMap<u32, Vec<Theme>>,
  involved: HashMap<u32, Vec<BookInvolved>>,
  characters: HashMap<u32, Vec<BookCharacter>>,
  mut statistics: HashMap<u32, BookStatistic>,
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
        genres.get(&(id as u32)).cloned().unwrap_or_default(),
        themes.get(&(id as u32)).cloned().unwrap_or_default(),
        involved.get(&(id as u32)).cloned().unwrap_or_default(),
        characters.get(&(id as u32)).cloned().unwrap_or_default(),
        statistics.remove(&(id as u32)).unwrap(),
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
fn fk_translation_in_ids(ids: &[i32]) -> Expression {
  Expression::new(ValueIn::new((DbBookTranslation::TABLE_NAME, "fktranslation"), ids))
}
fn no_translation_ids(book_ids: &[DbBook], translations: &[DbBookTranslation]) -> Vec<i32> {
  book_ids
    .iter()
    .filter_map(|x| {
      translations
        .iter()
        .find(|y| y.fk_translation == x.id)
        .map_or(Some(x.id), |_| None)
    })
    .collect()
}
