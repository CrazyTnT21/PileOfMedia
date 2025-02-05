use async_trait::async_trait;
use domain::available_translations::AvailableTranslations;
use std::collections::HashMap;
use std::error::Error;
use std::hash::Hash;
use std::sync::Arc;
use tokio_postgres::Client;

use crate::convert_to_sql::to_i32;
use crate::enums::db_language::DbLanguage;
use crate::schemas::db_person::DbPerson;
use crate::schemas::db_person_translation::DbPersonTranslation;
use crate::select::combined_tuple::CombinedType;
use crate::select::conditions::value_equal::ValueEqual;
use crate::select::conditions::value_ilike::ValueILike;
use crate::select::conditions::value_in::ValueIn;
use crate::select::expression::Expression;
use crate::select::Select;
use domain::entities::image::Image;
use domain::entities::person::person_translation::PersonTranslation;
use domain::entities::person::Person;
use domain::enums::language::Language;
use domain::items_total::ItemsTotal;
use domain::pagination::Pagination;
use from_row::{FromRow, Table};
use repositories::image_repository::ImageRepository;
use repositories::person_repository::PersonRepository;

pub struct DefaultPersonRepository<'a> {
  client: &'a Client,
  image_repository: Arc<dyn ImageRepository + 'a>,
}

impl<'a> DefaultPersonRepository<'a> {
  pub const fn new(client: &'a Client, image_repository: Arc<dyn ImageRepository + 'a>) -> DefaultPersonRepository<'a> {
    DefaultPersonRepository {
      client,
      image_repository,
    }
  }
}

#[async_trait]
impl PersonRepository for DefaultPersonRepository<'_> {
  async fn get(&self, languages: &[Language], pagination: Pagination) -> Result<ItemsTotal<Person>, Box<dyn Error>> {
    let db_languages: Vec<DbLanguage> = languages.iter().map(|x| (*x).into()).collect();
    let total = Select::new::<DbPerson>()
      .transform(inner_join_translation)
      .query_count(self.client)
      .await? as usize;

    let people = Select::new::<DbPerson>()
      .distinct_on(DbPerson::TABLE_NAME, "id")
      .columns_table::<DbPerson>()
      .transform(inner_join_translation)
      .pagination(pagination)
      .query_destruct(self.client)
      .await?;
    let person_ids: Vec<i32> = people.iter().map(|x| x.id).collect();

    let mut translations = person_translation_select(&person_ids, &db_languages)
      .query_destruct(self.client)
      .await?;

    let no_translations: Vec<i32> = no_translation_ids(&people, &translations);

    let mut extra_translations = Select::new::<DbPersonTranslation>()
      .distinct_on(DbPersonTranslation::TABLE_NAME, "fktranslation")
      .columns::<DbPersonTranslation>(DbPersonTranslation::TABLE_NAME)
      .where_expression(fk_translation_in_ids(&no_translations))
      .query_destruct(self.client)
      .await?;

    translations.append(&mut extra_translations);

    let translations: Vec<(Language, i32, PersonTranslation)> = translations
      .into_iter()
      .map(|x| (x.language.into(), x.fk_translation, x.to_entity()))
      .collect();

    let translations = map_translation(&people, translations);

    let image_ids: Vec<i32> = people.iter().filter_map(|x| x.fk_image).collect();
    let image_ids: Vec<u32> = to_u32(image_ids);
    let images = self.image_repository.get_by_ids(&image_ids).await?;

    let available = self.available_languages(&person_ids).await?;
    let people = to_entities(people, available, translations, images);
    Ok(ItemsTotal { items: people, total })
  }

  async fn get_by_id(&self, id: u32, languages: &[Language]) -> Result<Option<Person>, Box<dyn Error>> {
    let id = id as i32;
    let db_languages: Vec<DbLanguage> = languages.iter().map(|x| (*x).into()).collect();

    let people = Select::new::<DbPerson>()
      .columns_table::<DbPerson>()
      .distinct_on(DbPerson::TABLE_NAME, "id")
      .inner_join::<DbPersonTranslation>(
        None,
        Expression::new(ValueEqual::new((DbPerson::TABLE_NAME, "id"), id)).and(person_id_equal_fk_translation()),
      )
      .get_single_destruct(self.client)
      .await?;
    let Some(item) = people else {
      return Ok(None);
    };
    let mut translations = Select::new::<DbPersonTranslation>()
      .columns::<DbPersonTranslation>(DbPersonTranslation::TABLE_NAME)
      .where_expression(
        Expression::value_equal(DbPersonTranslation::TABLE_NAME, "fktranslation", item.id)
          .and(in_languages(&db_languages)),
      )
      .query_destruct(self.client)
      .await?;

    let people = [item];
    let no_translations: Vec<i32> = no_translation_ids(&people, &translations);
    let [item] = people;

    let mut extra_translations = Select::new::<DbPersonTranslation>()
      .distinct_on(DbPersonTranslation::TABLE_NAME, "fktranslation")
      .columns::<DbPersonTranslation>(DbPersonTranslation::TABLE_NAME)
      .where_expression(fk_translation_in_ids(&no_translations))
      .query_destruct(self.client)
      .await?;

    translations.append(&mut extra_translations);

    let translations: Vec<(Language, PersonTranslation)> = translations
      .into_iter()
      .map(|x| (x.language.into(), x.to_entity()))
      .collect();

    let mut available = self.available_languages(&[id]).await?;
    let image = match item.fk_image {
      None => None,
      Some(fk_image) => self.image_repository.get_by_id(fk_image as u32).await?,
    };
    let item = item.to_entity(
      AvailableTranslations {
        available_languages: available.remove(&id).unwrap(),
        translations: HashMap::from_iter(translations),
      },
      image,
    );
    Ok(Some(item))
  }

  async fn get_by_ids(&self, ids: &[u32], languages: &[Language]) -> Result<Vec<Person>, Box<dyn Error>> {
    let ids = to_i32(ids);
    let db_languages: Vec<DbLanguage> = languages.iter().map(|x| (*x).into()).collect();

    let people = Select::new::<DbPerson>()
      .columns_table::<DbPerson>()
      .distinct_on(DbPerson::TABLE_NAME, "id")
      .transform(inner_join_translation)
      .where_expression(Expression::new(ValueIn::new((DbPerson::TABLE_NAME, "id"), &ids)))
      .query_destruct(self.client)
      .await?;

    if people.is_empty() {
      return Ok(vec![]);
    }

    let person_ids: Vec<i32> = people.iter().map(|x| x.id).collect();

    let mut translations = person_translation_select(&person_ids, &db_languages)
      .query_destruct(self.client)
      .await?;

    let no_translations: Vec<i32> = no_translation_ids(&people, &translations);

    let mut extra_translations = Select::new::<DbPersonTranslation>()
      .distinct_on(DbPersonTranslation::TABLE_NAME, "fktranslation")
      .columns::<DbPersonTranslation>(DbPersonTranslation::TABLE_NAME)
      .where_expression(fk_translation_in_ids(&no_translations))
      .query_destruct(self.client)
      .await?;

    translations.append(&mut extra_translations);

    let translations: Vec<(Language, i32, PersonTranslation)> = translations
      .into_iter()
      .map(|x| (x.language.into(), x.fk_translation, x.to_entity()))
      .collect();

    let image_ids: Vec<i32> = people.iter().filter_map(|x| x.fk_image).collect();
    let image_ids: Vec<u32> = to_u32(image_ids);
    let images = self.image_repository.get_by_ids(&image_ids).await?;

    let translations = map_translation(&people, translations);
    let available = self.available_languages(&person_ids).await?;
    let people = to_entities(people, available, translations, images);
    Ok(people)
  }

  async fn get_by_name(
    &self,
    name: &str,
    languages: &[Language],
    pagination: Pagination,
  ) -> Result<ItemsTotal<Person>, Box<dyn Error>> {
    let name = format!("%{name}%");

    let db_languages: Vec<DbLanguage> = languages.iter().map(|x| (*x).into()).collect();
    let total = Select::new::<DbPerson>()
      .transform(|x| inner_join_translation_on_name(x, &name, &db_languages))
      .query_count(self.client)
      .await? as usize;

    let people = Select::new::<DbPerson>()
      .columns_table::<DbPerson>()
      .distinct_on(DbPerson::TABLE_NAME, "id")
      .transform(|x| inner_join_translation_on_name(x, &name, &db_languages))
      .pagination(pagination)
      .query_destruct(self.client)
      .await?;
    let person_ids: Vec<i32> = people.iter().map(|x| x.id).collect();

    let mut translations = person_translation_select(&person_ids, &db_languages)
      .where_expression(person_translation_with_name(&name))
      .query_destruct(self.client)
      .await?;

    let no_translations: Vec<i32> = no_translation_ids(&people, &translations);

    let mut extra_translations = Select::new::<DbPersonTranslation>()
      .distinct_on(DbPersonTranslation::TABLE_NAME, "fktranslation")
      .columns::<DbPersonTranslation>(DbPersonTranslation::TABLE_NAME)
      .where_expression(fk_translation_in_ids(&no_translations))
      .query_destruct(self.client)
      .await?;

    translations.append(&mut extra_translations);

    let translations: Vec<(Language, i32, PersonTranslation)> = translations
      .into_iter()
      .map(|x| (x.language.into(), x.fk_translation, x.to_entity()))
      .collect();

    let image_ids: Vec<i32> = people.iter().filter_map(|x| x.fk_image).collect();
    let image_ids: Vec<u32> = to_u32(image_ids);
    let images = self.image_repository.get_by_ids(&image_ids).await?;

    let translations = map_translation(&people, translations);
    let available = self.available_languages(&person_ids).await?;
    let people = to_entities(people, available, translations, images);
    Ok(ItemsTotal { items: people, total })
  }

  async fn filter_existing(&self, people: &[u32]) -> Result<Vec<u32>, Box<dyn Error>> {
    let people = to_i32(people);

    let count = Select::new::<DbPerson>()
      .column::<i32>(DbPerson::TABLE_NAME, "id")
      .where_expression(id_in_ids(&people))
      .query(self.client)
      .await?
      .into_iter()
      .map(|x| x.0 as u32)
      .collect();
    Ok(count)
  }
}
impl DefaultPersonRepository<'_> {
  async fn available_languages(&self, ids: &[i32]) -> Result<HashMap<i32, Vec<Language>>, Box<dyn Error>> {
    let available_translations = Select::new::<DbPersonTranslation>()
      .column::<i32>(DbPersonTranslation::TABLE_NAME, "fktranslation")
      .column::<DbLanguage>(DbPersonTranslation::TABLE_NAME, "language")
      .where_expression(Expression::new(ValueIn::new(
        (DbPersonTranslation::TABLE_NAME, "fktranslation"),
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
fn person_translation_select<'a>(
  person_ids: &'a [i32],
  db_languages: &'a [DbLanguage],
) -> Select<'a, (DbPersonTranslation,)> {
  Select::new::<DbPersonTranslation>()
    .columns::<DbPersonTranslation>(DbPersonTranslation::TABLE_NAME)
    .where_expression(Expression::new(ValueIn::new(
      (DbPersonTranslation::TABLE_NAME, "fktranslation"),
      person_ids,
    )))
    .where_expression(in_languages(db_languages))
}
fn map_translation(
  people: &[DbPerson],
  translations: Vec<(Language, i32, PersonTranslation)>,
) -> HashMap<i32, Vec<(Language, PersonTranslation)>> {
  let mut new_translations: HashMap<i32, Vec<(Language, PersonTranslation)>> = HashMap::new();
  for person in people {
    new_translations.insert(person.id, vec![]);
  }
  for (language, id, translation) in translations {
    let result = new_translations.get_mut(&id).unwrap();
    result.push((language, translation));
  }
  new_translations
}
fn person_id_equal_fk_translation<'a>() -> Expression<'a> {
  Expression::column_equal(
    (DbPerson::TABLE_NAME, "id"),
    (DbPersonTranslation::TABLE_NAME, "fktranslation"),
  )
}
fn person_translation_with_name(name: &String) -> Expression {
  Expression::new(ValueILike::new((DbPersonTranslation::TABLE_NAME, "name"), name))
}
fn inner_join_translation_on_name<'a, T: FromRow<DbType = T> + CombinedType>(
  select: Select<'a, T>,
  name: &'a String,
  db_languages: &'a [DbLanguage],
) -> Select<'a, T> {
  select.inner_join::<DbPersonTranslation>(
    None,
    Expression::value_i_like((DbPersonTranslation::TABLE_NAME, "name"), name)
      .and(in_languages(db_languages))
      .and(person_id_equal_fk_translation()),
  )
}
fn to_entities(
  people: Vec<DbPerson>,
  mut available: HashMap<i32, Vec<Language>>,
  mut translations: HashMap<i32, Vec<(Language, PersonTranslation)>>,
  mut images: Vec<Image>,
) -> Vec<Person> {
  people
    .into_iter()
    .map(|person| {
      let id = person.id;
      let image_id = person.fk_image;
      let image = image_id.and_then(|x| images.iter().position(|y| y.id == x as u32).map(|x| images.remove(x)));
      person.to_entity(
        AvailableTranslations {
          available_languages: available.remove(&id).unwrap(),
          translations: HashMap::from_iter(translations.remove(&id).unwrap()),
        },
        image,
      )
    })
    .collect()
}
fn inner_join_translation<T: FromRow<DbType = T> + CombinedType>(select: Select<T>) -> Select<T> {
  select.inner_join::<DbPersonTranslation>(None, person_id_equal_fk_translation())
}
fn in_languages(languages: &[DbLanguage]) -> Expression {
  Expression::new(ValueIn::new((DbPersonTranslation::TABLE_NAME, "language"), languages))
}
fn to_u32(values: Vec<i32>) -> Vec<u32> {
  values.into_iter().map(|x| x as u32).collect()
}
fn fk_translation_in_ids(ids: &[i32]) -> Expression {
  Expression::new(ValueIn::new((DbPersonTranslation::TABLE_NAME, "fktranslation"), ids))
}
fn id_in_ids(ids: &[i32]) -> Expression {
  Expression::new(ValueIn::new((DbPerson::TABLE_NAME, "id"), ids))
}
fn no_translation_ids(person_ids: &[DbPerson], translations: &[DbPersonTranslation]) -> Vec<i32> {
  person_ids
    .iter()
    .filter_map(|x| {
      translations
        .iter()
        .find(|y| y.fk_translation == x.id)
        .map_or(Some(x.id), |_| None)
    })
    .collect()
}
