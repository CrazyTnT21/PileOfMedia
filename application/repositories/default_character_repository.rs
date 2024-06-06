use std::error::Error;

use async_trait::async_trait;

use domain::entities::character::Character;
use domain::entities::image::Image;
use domain::enums::language::Language;
use domain::items_total::ItemsTotal;
use domain::pagination::Pagination;
use from_row::FromRow;
use repositories::character_repository::CharacterRepository;
use repositories::image_repository::ImageRepository;

use crate::convert_to_sql::convert_to_sql;
use crate::enums::db_language::DbLanguage;
use crate::fallback_unwrap::fallback_unwrap;
use crate::Pooled;
use crate::schemas::db_character::DbCharacter;
use crate::schemas::db_character_translation::DbCharacterTranslation;
use crate::select::combined_tuple::CombinedType;
use crate::select::comparison::Comparison::{Equal, ILike, In};
use crate::select::condition::Condition::{Column, Value};
use crate::select::expression::Expression;
use crate::select::Select;

pub struct DefaultCharacterRepository<'a> {
  pool: &'a Pooled<'a>,
  default_language: DbLanguage,
  image_repository: &'a dyn ImageRepository,
}

impl<'a> DefaultCharacterRepository<'a> {
  pub fn new(pool: &'a Pooled, language: Language, image_repository: &'a dyn ImageRepository) -> DefaultCharacterRepository<'a> {
    DefaultCharacterRepository { pool, default_language: language.into(), image_repository }
  }
}

#[async_trait]
impl<'a> CharacterRepository for DefaultCharacterRepository<'a> {
  async fn get(&self, language: Language, pagination: Pagination) -> Result<ItemsTotal<Character>, Box<dyn Error>> {
    let language = DbLanguage::from(language);
    let select = character_select_columns()
      .transform(|x| self.character_joins(x, &language));

    let total = select.count(self.pool).await? as usize;

    let characters = select
      .pagination(pagination)
      .query(self.pool)
      .await?;

    let characters = self.to_entities(characters).await?;

    Ok(ItemsTotal { items: characters, total })
  }

  async fn get_by_id(&self, id: u32, language: Language) -> Result<Option<Character>, Box<dyn Error>> {
    let id = id as i32;
    let language = DbLanguage::from(language);
    let character = character_select_columns()
      .transform(|x| self.character_joins(x, &language))
      .where_expression(Expression::new(Value(("character", "id"), Equal(&id))))
      .get_single(self.pool)
      .await?;

    let fk_image = character
      .as_ref()
      .and_then(|x| x.0.fk_image);

    let image = match fk_image {
      None => None,
      Some(x) => Some(self.image_repository.get_by_id(x as u32).await?.unwrap())
    };

    Ok(character.map(|x| to_entity(x, image)))
  }

  async fn get_by_ids(&self, ids: &[i32], language: Language) -> Result<Vec<Character>, Box<dyn Error>> {
    let language = DbLanguage::from(language);
    let ids = convert_to_sql(ids);
    let characters = character_select_columns()
      .transform(|x| self.character_joins(x, &language))
      .where_expression(Expression::new(Value(("character", "id"), In(&ids))))
      .query(self.pool)
      .await?;

    let characters = self.to_entities(characters).await?;

    Ok(characters)
  }

  async fn get_by_name(&self, name: &str, language: Language, pagination: Pagination) -> Result<ItemsTotal<Character>, Box<dyn Error>> {
    let language = DbLanguage::from(language);
    let name = format!("%{name}%");
    let select = character_select_columns()
      .transform(|x| self.character_joins(x, &language))
      .where_expression(Expression::new(Value(("character_translation", "name"), ILike(&name)))
        .or(Expression::new(Value(("character_translation_fallback", "name"), ILike(&name)))));

    let total = select.count(self.pool).await? as usize;

    let characters = select
      .pagination(pagination)
      .query(self.pool)
      .await?;

    let characters = self.to_entities(characters).await?;

    Ok(ItemsTotal { items: characters, total })
  }
}

impl<'a> DefaultCharacterRepository<'a> {
  fn character_joins<T: FromRow<DbType=T> + CombinedType>(&'a self, select: Select<'a, T>, language: &'a DbLanguage) -> Select<'a, T> {
    select
      .left_join::<DbCharacterTranslation>(
        Some("character_translation"),
        Expression::column_equal("character_translation", "language", language)
          .and(Expression::new(Column(("character_translation", "fktranslation"), ("character", "id")))),
      )
      .left_join::<DbCharacterTranslation>(
        Some("character_translation_fallback"),
        Expression::column_equal("character_translation_fallback", "language", &self.default_language)
          .and(Expression::new(Column(("character_translation_fallback", "fktranslation"), ("character", "id"))))
          .and(Expression::column_null("character_translation", "fktranslation")),
      )
  }
  async fn to_entities(&self, items: Vec<CharacterColumns>) -> Result<Vec<Character>, Box<dyn Error>> {
    if items.is_empty() {
      return Ok(vec![]);
    }

    let image_ids = image_ids(&items);
    let mut images = match image_ids.is_empty() {
      true => vec![],
      false => self.image_repository.get_by_ids(&image_ids).await?
    };


    Ok(items
      .into_iter()
      .map(|x| {
        let fk_image = x.0.fk_image;
        let image = get_image(fk_image, &mut images);
        to_entity(x, image)
      })
      .collect())
  }
}

fn to_entity(character: (DbCharacter, Option<DbCharacterTranslation>, Option<DbCharacterTranslation>), image: Option<Image>) -> Character {
  character.0.to_entity(fallback_unwrap(character.1, character.2), image)
}

fn character_select_columns<'a>() -> Select<'a, CharacterColumns> {
  Select::new::<DbCharacter>()
    .columns::<DbCharacter>("character")
    .columns::<Option<DbCharacterTranslation>>("character_translation")
    .columns::<Option<DbCharacterTranslation>>("character_translation_fallback")
}

fn get_image(fk_image: Option<i32>, images: &mut Vec<Image>) -> Option<Image> {
  let fk_image = fk_image?;
  let index = images.iter().position(|x| x.id == fk_image);
  index.map(|index| images.swap_remove(index))
}

fn image_ids(items: &[CharacterColumns]) -> Vec<i32> {
  items
    .iter()
    .filter_map(|x| x.0.fk_image)
    .collect::<Vec<i32>>()
}


type CharacterColumns = (DbCharacter, Option<DbCharacterTranslation>, Option<DbCharacterTranslation>);
