use std::error::Error;

use async_trait::async_trait;

use domain::entities::image::Image;
use domain::entities::person::Person;
use domain::enums::language::Language;
use domain::items_total::ItemsTotal;
use domain::pagination::Pagination;
use from_row::FromRow;
use repositories::image_repository::ImageRepository;
use repositories::person_repository::PersonRepository;

use crate::convert_to_sql::convert_to_sql;
use crate::enums::db_language::DbLanguage;
use crate::fallback_unwrap::fallback_unwrap;
use crate::Pooled;
use crate::schemas::db_person::DbPerson;
use crate::schemas::db_person_translation::DbPersonTranslation;
use crate::select::combined_tuple::CombinedType;
use crate::select::comparison::Comparison::{Equal, ILike, In};
use crate::select::condition::Condition::{Column, Value};
use crate::select::expression::Expression;
use crate::select::Select;

pub struct DefaultPersonRepository<'a> {
  pool: &'a Pooled<'a>,
  default_language: DbLanguage,
  image_repository: &'a dyn ImageRepository,
}

impl<'a> DefaultPersonRepository<'a> {
  pub fn new(pool: &'a Pooled, language: Language, image_repository: &'a dyn ImageRepository) -> DefaultPersonRepository<'a> {
    DefaultPersonRepository { pool, default_language: language.into(), image_repository }
  }
}

#[async_trait]
impl<'a> PersonRepository for DefaultPersonRepository<'a> {
  async fn get(&self, language: Language, pagination: Pagination) -> Result<ItemsTotal<Person>, Box<dyn Error>> {
    let language = DbLanguage::from(language);
    let select = person_select_columns()
      .transform(|x| self.person_joins(x, &language));

    let total = select.count(self.pool).await? as usize;

    let people = select
      .pagination(pagination)
      .query(self.pool)
      .await?;

    let mut images = match people.len() {
      0 => vec![],
      _ => self.image_repository.get_by_ids(&image_ids(&people)).await?
    };
    let people =
      people.into_iter()
        .map(|x| {
          let fk = x.0.fk_image;
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
      .get_single(self.pool)
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

  async fn get_by_ids(&self, ids: &[i32], language: Language) -> Result<Vec<Person>, Box<dyn Error>> {
    let language = DbLanguage::from(language);
    let ids = convert_to_sql(ids);
    let people = person_select_columns()
      .transform(|x| self.person_joins(x, &language))
      .where_expression(Expression::new(Value(("person", "id"), In(&ids))))
      .query(self.pool)
      .await?;
    let mut images = match people.len() {
      0 => vec![],
      _ => self.image_repository.get_by_ids(&image_ids(&people)).await?
    };
    let people =
      people.into_iter()
        .map(|x| {
          let fk = x.0.fk_image;
          to_entity(x, get_image(fk, &mut images))
        })
        .collect();

    Ok(people)
  }

  async fn get_by_name(&self, name: &str, language: Language, pagination: Pagination) -> Result<ItemsTotal<Person>, Box<dyn Error>> {
    let language = DbLanguage::from(language);
    let name = format!("%{name}%");
    let select = person_select_columns()
      .transform(|x| self.person_joins(x, &language))
      .where_expression(Expression::new(Value(("person", "name"), ILike(&name))));

    let total = select.count(self.pool).await? as usize;

    let people = select
      .pagination(pagination)
      .query(self.pool)
      .await?;

    let mut images = match people.len() {
      0 => vec![],
      _ => self.image_repository.get_by_ids(&image_ids(&people)).await?
    };
    let people =
      people.into_iter()
        .map(|x| {
          let fk = x.0.fk_image;
          to_entity(x, get_image(fk, &mut images))
        })
        .collect();

    Ok(ItemsTotal { items: people, total })
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

fn get_image(fk_image: Option<i32>, images: &mut Vec<Image>) -> Option<Image> {
  let fk_image = fk_image?;
  let index = images.iter().position(|x| x.id == fk_image);
  index.map(|x| images.swap_remove(x))
}


fn person_select_columns<'a>() -> Select<'a, PersonColumns> {
  Select::new("person")
    .columns::<DbPerson>("person")
    .columns::<Option<DbPersonTranslation>>("person_translation")
    .columns::<Option<DbPersonTranslation>>("person_translation_fallback")
}

fn image_ids(items: &[PersonColumns]) -> Vec<i32> {
  items
    .iter()
    .filter_map(|x| x.0.fk_image)
    .collect::<Vec<i32>>()
}

type PersonColumns = (DbPerson, Option<DbPersonTranslation>, Option<DbPersonTranslation>);
