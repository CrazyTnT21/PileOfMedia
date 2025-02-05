use async_trait::async_trait;
use domain::available_translations::AvailableTranslations;
use std::collections::HashMap;
use std::error::Error;
use std::hash::Hash;
use tokio_postgres::Client;

use domain::entities::genre::genre_translation::GenreTranslation;
use domain::entities::genre::Genre;
use domain::enums::language::Language;
use domain::items_total::ItemsTotal;
use domain::pagination::Pagination;
use from_row::{FromRow, Table};
use repositories::genre_repository::GenreRepository;

use crate::convert_to_sql::to_i32;
use crate::enums::db_language::DbLanguage;
use crate::schemas::db_genre::DbGenre;
use crate::schemas::db_genre_translation::DbGenreTranslation;
use crate::select::combined_tuple::CombinedType;
use crate::select::conditions::value_equal::ValueEqual;
use crate::select::conditions::value_ilike::ValueILike;
use crate::select::conditions::value_in::ValueIn;
use crate::select::expression::Expression;
use crate::select::Select;

pub struct DefaultGenreRepository<'a> {
  client: &'a Client,
}

impl<'a> DefaultGenreRepository<'a> {
  pub const fn new(client: &'a Client) -> DefaultGenreRepository<'a> {
    DefaultGenreRepository { client }
  }
}

#[async_trait]
impl GenreRepository for DefaultGenreRepository<'_> {
  async fn get(&self, languages: &[Language], pagination: Pagination) -> Result<ItemsTotal<Genre>, Box<dyn Error>> {
    let db_languages: Vec<DbLanguage> = languages.iter().map(|x| (*x).into()).collect();
    let total = Select::new::<DbGenre>()
      .transform(inner_join_translation)
      .query_count(self.client)
      .await? as usize;

    let genres = Select::new::<DbGenre>()
      .distinct_on(DbGenre::TABLE_NAME, "id")
      .columns_table::<DbGenre>()
      .transform(inner_join_translation)
      .pagination(pagination)
      .query_destruct(self.client)
      .await?;
    let genre_ids: Vec<i32> = genres.iter().map(|x| x.id).collect();

    let mut translations: Vec<DbGenreTranslation> = genre_translation_select(&genre_ids, &db_languages)
      .query_destruct(self.client)
      .await?;

    let no_translations: Vec<i32> = no_translation_ids(&genres, &translations);

    let mut extra_translations = Select::new::<DbGenreTranslation>()
      .distinct_on(DbGenreTranslation::TABLE_NAME, "fktranslation")
      .columns::<DbGenreTranslation>(DbGenreTranslation::TABLE_NAME)
      .where_expression(fk_translation_in_ids(&no_translations))
      .query_destruct(self.client)
      .await?;

    translations.append(&mut extra_translations);

    let translations: Vec<(Language, i32, GenreTranslation)> = translations
      .into_iter()
      .map(|x| (x.language.into(), x.fk_translation, x.to_entity()))
      .collect();

    let available = self.available_languages(&genre_ids).await?;
    let translations = map_translation(&genres, translations);

    let genres = to_entities(genres, available, translations);
    Ok(ItemsTotal { items: genres, total })
  }

  async fn get_by_id(&self, id: u32, languages: &[Language]) -> Result<Option<Genre>, Box<dyn Error>> {
    let id = id as i32;
    let db_languages: Vec<DbLanguage> = languages.iter().map(|x| (*x).into()).collect();

    let genres = Select::new::<DbGenre>()
      .columns_table::<DbGenre>()
      .distinct_on(DbGenre::TABLE_NAME, "id")
      .inner_join::<DbGenreTranslation>(
        None,
        Expression::new(ValueEqual::new((DbGenre::TABLE_NAME, "id"), id)).and(genre_id_equal_fk_translation()),
      )
      .get_single_destruct(self.client)
      .await?;
    let Some(item) = genres else {
      return Ok(None);
    };
    let mut translations = Select::new::<DbGenreTranslation>()
      .columns::<DbGenreTranslation>(DbGenreTranslation::TABLE_NAME)
      .where_expression(
        Expression::value_equal(DbGenreTranslation::TABLE_NAME, "fktranslation", item.id)
          .and(in_languages(&db_languages)),
      )
      .query_destruct(self.client)
      .await?;
    let items = [item];
    let no_translations: Vec<i32> = no_translation_ids(&items, &translations);
    let [item] = items;

    let mut extra_translations = Select::new::<DbGenreTranslation>()
      .distinct_on(DbGenreTranslation::TABLE_NAME, "fktranslation")
      .columns::<DbGenreTranslation>(DbGenreTranslation::TABLE_NAME)
      .where_expression(fk_translation_in_ids(&no_translations))
      .query_destruct(self.client)
      .await?;

    translations.append(&mut extra_translations);

    let translations: Vec<(Language, GenreTranslation)> = translations
      .into_iter()
      .map(|x| (x.language.into(), x.to_entity()))
      .collect();

    let mut available = self.available_languages(&[id]).await?;
    let item = item.to_entity(AvailableTranslations {
      available_languages: available.remove(&id).unwrap(),
      translations: HashMap::from_iter(translations),
    });
    Ok(Some(item))
  }

  async fn get_by_ids(&self, ids: &[u32], languages: &[Language]) -> Result<Vec<Genre>, Box<dyn Error>> {
    let ids = to_i32(ids);
    let db_languages: Vec<DbLanguage> = languages.iter().map(|x| (*x).into()).collect();

    let genres = Select::new::<DbGenre>()
      .columns_table::<DbGenre>()
      .distinct_on(DbGenre::TABLE_NAME, "id")
      .transform(inner_join_translation)
      .where_expression(id_in_ids(&ids))
      .query_destruct(self.client)
      .await?;

    if genres.is_empty() {
      return Ok(vec![]);
    }
    let genre_ids: Vec<i32> = genres.iter().map(|x| x.id).collect();

    let mut translations = genre_translation_select(&genre_ids, &db_languages)
      .query_destruct(self.client)
      .await?;

    let no_translations: Vec<i32> = no_translation_ids(&genres, &translations);

    let mut extra_translations = Select::new::<DbGenreTranslation>()
      .distinct_on(DbGenreTranslation::TABLE_NAME, "fktranslation")
      .columns::<DbGenreTranslation>(DbGenreTranslation::TABLE_NAME)
      .where_expression(fk_translation_in_ids(&no_translations))
      .query_destruct(self.client)
      .await?;

    translations.append(&mut extra_translations);

    let translations: Vec<(Language, i32, GenreTranslation)> = translations
      .into_iter()
      .map(|x| (x.language.into(), x.fk_translation, x.to_entity()))
      .collect();

    let translations = map_translation(&genres, translations);
    let available = self.available_languages(&genre_ids).await?;
    let genres = to_entities(genres, available, translations);
    Ok(genres)
  }

  async fn get_by_name(
    &self,
    name: &str,
    languages: &[Language],
    pagination: Pagination,
  ) -> Result<ItemsTotal<Genre>, Box<dyn Error>> {
    let name = format!("%{name}%");

    let db_languages: Vec<DbLanguage> = languages.iter().map(|x| (*x).into()).collect();
    let total = Select::new::<DbGenre>()
      .transform(|x| inner_join_translation_on_name(x, &name, &db_languages))
      .query_count(self.client)
      .await? as usize;

    let genres = Select::new::<DbGenre>()
      .columns_table::<DbGenre>()
      .distinct_on(DbGenre::TABLE_NAME, "id")
      .transform(|x| inner_join_translation_on_name(x, &name, &db_languages))
      .pagination(pagination)
      .query_destruct(self.client)
      .await?;
    let genre_ids: Vec<i32> = genres.iter().map(|x| x.id).collect();

    let mut translations = genre_translation_select(&genre_ids, &db_languages)
      .where_expression(genre_translation_with_name(&name))
      .query_destruct(self.client)
      .await?;

    let no_translations: Vec<i32> = no_translation_ids(&genres, &translations);

    let mut extra_translations = Select::new::<DbGenreTranslation>()
      .distinct_on(DbGenreTranslation::TABLE_NAME, "fktranslation")
      .columns::<DbGenreTranslation>(DbGenreTranslation::TABLE_NAME)
      .where_expression(fk_translation_in_ids(&no_translations))
      .query_destruct(self.client)
      .await?;

    translations.append(&mut extra_translations);

    let translations: Vec<(Language, i32, GenreTranslation)> = translations
      .into_iter()
      .map(|x| (x.language.into(), x.fk_translation, x.to_entity()))
      .collect();

    let translations = map_translation(&genres, translations);
    let available = self.available_languages(&genre_ids).await?;
    let genres = to_entities(genres, available, translations);
    Ok(ItemsTotal { items: genres, total })
  }

  async fn filter_existing(&self, genres: &[u32]) -> Result<Vec<u32>, Box<dyn Error>> {
    let genres = to_i32(genres);

    let count = Select::new::<DbGenre>()
      .column::<i32>(DbGenre::TABLE_NAME, "id")
      .where_expression(id_in_ids(&genres))
      .query(self.client)
      .await?
      .into_iter()
      .map(|x| x.0 as u32)
      .collect();
    Ok(count)
  }
}
impl DefaultGenreRepository<'_> {
  async fn available_languages(&self, ids: &[i32]) -> Result<HashMap<i32, Vec<Language>>, Box<dyn Error>> {
    let available_translations = Select::new::<DbGenreTranslation>()
      .column::<i32>(DbGenreTranslation::TABLE_NAME, "fktranslation")
      .column::<DbLanguage>(DbGenreTranslation::TABLE_NAME, "language")
      .where_expression(Expression::new(ValueIn::new(
        (DbGenreTranslation::TABLE_NAME, "fktranslation"),
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
fn genre_translation_select<'a>(
  genre_ids: &'a [i32],
  db_languages: &'a [DbLanguage],
) -> Select<'a, (DbGenreTranslation,)> {
  Select::new::<DbGenreTranslation>()
    .columns::<DbGenreTranslation>(DbGenreTranslation::TABLE_NAME)
    .where_expression(fk_translation_in_ids(genre_ids))
    .where_expression(in_languages(db_languages))
}
fn map_translation(
  genres: &[DbGenre],
  translations: Vec<(Language, i32, GenreTranslation)>,
) -> HashMap<i32, Vec<(Language, GenreTranslation)>> {
  let mut new_translations: HashMap<i32, Vec<(Language, GenreTranslation)>> = HashMap::new();
  for genre in genres {
    new_translations.insert(genre.id, vec![]);
  }
  for (language, id, translation) in translations {
    let result = new_translations.get_mut(&id).unwrap();
    result.push((language, translation));
  }
  new_translations
}
fn genre_id_equal_fk_translation<'a>() -> Expression<'a> {
  Expression::column_equal(
    (DbGenre::TABLE_NAME, "id"),
    (DbGenreTranslation::TABLE_NAME, "fktranslation"),
  )
}
fn genre_translation_with_name(name: &String) -> Expression {
  Expression::new(ValueILike::new((DbGenreTranslation::TABLE_NAME, "name"), name))
}
fn inner_join_translation_on_name<'a, T: FromRow<DbType = T> + CombinedType>(
  select: Select<'a, T>,
  name: &'a String,
  db_languages: &'a [DbLanguage],
) -> Select<'a, T> {
  select.inner_join::<DbGenreTranslation>(
    None,
    Expression::value_i_like((DbGenreTranslation::TABLE_NAME, "name"), name)
      .and(in_languages(db_languages))
      .and(genre_id_equal_fk_translation()),
  )
}
fn to_entities(
  genres: Vec<DbGenre>,
  mut available: HashMap<i32, Vec<Language>>,
  mut translations: HashMap<i32, Vec<(Language, GenreTranslation)>>,
) -> Vec<Genre> {
  genres
    .into_iter()
    .map(|genre| {
      let id = genre.id;
      genre.to_entity(AvailableTranslations {
        available_languages: available.remove(&id).unwrap(),
        translations: HashMap::from_iter(translations.remove(&id).unwrap()),
      })
    })
    .collect()
}
fn inner_join_translation<T: FromRow<DbType = T> + CombinedType>(select: Select<T>) -> Select<T> {
  select.inner_join::<DbGenreTranslation>(None, genre_id_equal_fk_translation())
}
fn in_languages(languages: &[DbLanguage]) -> Expression {
  Expression::new(ValueIn::new((DbGenreTranslation::TABLE_NAME, "language"), languages))
}
fn fk_translation_in_ids(ids: &[i32]) -> Expression {
  Expression::new(ValueIn::new((DbGenreTranslation::TABLE_NAME, "fktranslation"), ids))
}
fn id_in_ids(ids: &[i32]) -> Expression {
  Expression::new(ValueIn::new((DbGenre::TABLE_NAME, "id"), ids))
}
fn no_translation_ids(genre_ids: &[DbGenre], translations: &[DbGenreTranslation]) -> Vec<i32> {
  genre_ids
    .iter()
    .filter_map(|x| {
      translations
        .iter()
        .find(|y| y.fk_translation == x.id)
        .map_or(Some(x.id), |_| None)
    })
    .collect()
}
//TODO NonEmptyHashMap
