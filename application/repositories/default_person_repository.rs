use std::error::Error;
use std::sync::Arc;

use async_trait::async_trait;
use tokio_postgres::Client;

use domain::entities::image::Image;
use domain::entities::person::Person;
use domain::enums::language::Language;
use domain::items_total::ItemsTotal;
use domain::pagination::Pagination;
use from_row::{FromRow, Table};
use repositories::image_repository::ImageRepository;
use repositories::person_repository::PersonRepository;

use crate::convert_to_sql::{convert_to_sql, to_i32};
use crate::enums::db_language::DbLanguage;
use crate::fallback_unwrap::fallback_unwrap;
use crate::schemas::db_person::DbPerson;
use crate::schemas::db_person_translation::DbPersonTranslation;
use crate::select::combined_tuple::CombinedType;
use crate::select::comparison::Comparison::{Equal, ILike, In};
use crate::select::condition::Condition::{Column, Value};
use crate::select::expression::Expression;
use crate::select::Select;

pub struct DefaultPersonRepository<'a> {
  client: &'a Client,
  default_language: DbLanguage,
  image_repository: Arc<dyn ImageRepository + 'a>,
}

impl<'a> DefaultPersonRepository<'a> {
  pub fn new(client: &'a Client, language: Language, image_repository: Arc<dyn ImageRepository + 'a>) -> DefaultPersonRepository<'a> {
    DefaultPersonRepository { client, default_language: language.into(), image_repository }
  }
}

#[async_trait]
impl<'a> PersonRepository for DefaultPersonRepository<'a> {
  async fn get(&self, language: Language, pagination: Pagination) -> Result<ItemsTotal<Person>, Box<dyn Error>> {
    let language = DbLanguage::from(language);

    let total = Select::new::<DbPerson>()
      .count()
      .transform(|x| self.person_joins(x, &language))
      .get_single(self.client).await?
      .expect("Count should return one row");
    let total = total.0 as usize;

    let people = person_select_columns()
      .transform(|x| self.person_joins(x, &language))
      .pagination(pagination)
      .query(self.client)
      .await?;

    let image_ids = image_ids(&people);
    let mut images = match image_ids.is_empty() {
      true => vec![],
      false => self.image_repository.get_by_ids(&image_ids).await?
    };

    let people =
      people.into_iter()
        .map(|x| {
          let fk = x.0.fk_image.map(|x| x as u32);
          to_entity(x, get_image(fk, &mut images))
        })
        .collect();

    Ok(ItemsTotal { items: people, total })
  }

  async fn get_by_id(&self, id: u32, language: Language) -> Result<Option<Person>, Box<dyn Error>> {
    let id = id as i32;
    let language = DbLanguage::from(language);
    let person = person_select_columns()
      .transform(|x| self.person_joins(x, &language))
      .where_expression(Expression::new(Value(("person", "id"), Equal(&id))))
      .get_single(self.client)
      .await?;
    let image = match person {
      None => None,
      Some(ref x) => match x.0.fk_image {
        None => None,
        Some(fk) => self.image_repository.get_by_id(fk as u32).await?
      }
    };
    Ok(person.map(|x| {
      to_entity(x, image)
    }))
  }

  async fn get_by_ids(&self, ids: &[u32], language: Language) -> Result<Vec<Person>, Box<dyn Error>> {
    let language = DbLanguage::from(language);
    let ids = to_i32(ids);
    let ids = convert_to_sql(&ids);
    let people = person_select_columns()
      .transform(|x| self.person_joins(x, &language))
      .where_expression(Expression::new(Value(("person", "id"), In(&ids))))
      .query(self.client)
      .await?;

    let image_ids = image_ids(&people);
    let mut images = match image_ids.is_empty() {
      true => vec![],
      false => self.image_repository.get_by_ids(&image_ids).await?
    };
    let people =
      people.into_iter()
        .map(|x| {
          let fk = x.0.fk_image.map(|x| x as u32);
          to_entity(x, get_image(fk, &mut images))
        })
        .collect();

    Ok(people)
  }

  async fn get_by_name(&self, name: &str, language: Language, pagination: Pagination) -> Result<ItemsTotal<Person>, Box<dyn Error>> {
    let language = DbLanguage::from(language);
    let name = format!("%{name}%");

    let total = Select::new::<DbPerson>()
      .count()
      .transform(|x| self.person_joins(x, &language))
      .where_expression(Expression::new(Value(("person", "name"), ILike(&name))))
      .get_single(self.client).await?
      .expect("Count should return one row");
    let total = total.0 as usize;

    let people = person_select_columns()
      .transform(|x| self.person_joins(x, &language))
      .where_expression(Expression::new(Value(("person", "name"), ILike(&name))))
      .pagination(pagination)
      .query(self.client)
      .await?;

    let image_ids = image_ids(&people);
    let mut images = match image_ids.is_empty() {
      true => vec![],
      false => self.image_repository.get_by_ids(&image_ids).await?
    };

    let people =
      people.into_iter()
        .map(|x| {
          let fk = x.0.fk_image.map(|x| x as u32);
          to_entity(x, get_image(fk, &mut images))
        })
        .collect();

    Ok(ItemsTotal { items: people, total })
  }

  async fn filter_existing(&self, people: &[u32]) -> Result<Vec<u32>, Box<dyn Error>> {
    let people = to_i32(people);
    let people = convert_to_sql(&people);
    let count = Select::new::<DbPerson>()
      .column::<i32>(DbPerson::TABLE_NAME, "id")
      .where_expression(Expression::new(Value((DbPerson::TABLE_NAME, "id"), In(&people))))
      .query(self.client)
      .await?
      .into_iter()
      .map(|x| { x.0 as u32 })
      .collect();
    Ok(count)
  }
}

impl<'a> DefaultPersonRepository<'a> {
  fn person_joins<T: FromRow<DbType=T> + CombinedType>(&'a self, select: Select<'a, T>, language: &'a DbLanguage) -> Select<'a, T> {
    select
      .left_join::<DbPersonTranslation>(
        Some("person_translation"),
        Expression::column_equal("person_translation", "language", language)
          .and(Expression::new(Column(("person_translation", "fktranslation"), ("person", "id")))),
      )
      .left_join::<DbPersonTranslation>(
        Some("person_translation_fallback"),
        Expression::column_equal("person_translation_fallback", "language", &self.default_language)
          .and(Expression::new(Column(("person_translation_fallback", "fktranslation"), ("person", "id"))))
          .and(Expression::column_null("person_translation", "fktranslation")),
      )
  }
}

fn to_entity(person: (DbPerson, Option<DbPersonTranslation>, Option<DbPersonTranslation>), image: Option<Image>) -> Person {
  person.0.to_entity(fallback_unwrap(person.1, person.2), image)
}

fn get_image(fk_image: Option<u32>, images: &mut Vec<Image>) -> Option<Image> {
  let fk_image = fk_image?;
  let index = images.iter().position(|x| x.id == fk_image);
  index.map(|x| images.swap_remove(x))
}


fn person_select_columns<'a>() -> Select<'a, PersonColumns> {
  Select::new::<DbPerson>()
    .columns::<DbPerson>("person")
    .columns::<Option<DbPersonTranslation>>("person_translation")
    .columns::<Option<DbPersonTranslation>>("person_translation_fallback")
}

fn image_ids(items: &[PersonColumns]) -> Vec<u32> {
  items
    .iter()
    .filter_map(|x| x.0.fk_image.map(|x| x as u32))
    .collect()
}

type PersonColumns = (DbPerson, Option<DbPersonTranslation>, Option<DbPersonTranslation>);
