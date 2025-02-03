use std::error::Error;
use std::sync::Arc;

use async_trait::async_trait;
use tokio_postgres::Client;

use domain::entities::character::Character;
use domain::entities::image::Image;
use domain::enums::language::Language;
use domain::items_total::ItemsTotal;
use domain::pagination::Pagination;
use from_row::{FromRow, Table};
use repositories::character_repository::CharacterRepository;
use repositories::image_repository::ImageRepository;

use crate::convert_to_sql::to_i32;
use crate::enums::db_language::DbLanguage;
use crate::fallback_unwrap::fallback_unwrap;
use crate::schemas::db_character::DbCharacter;
use crate::schemas::db_character_translation::DbCharacterTranslation;
use crate::select::combined_tuple::CombinedType;
use crate::select::conditions::column_equal::ColumnEqual;
use crate::select::conditions::column_null::ColumnNull;
use crate::select::conditions::value_equal::ValueEqual;
use crate::select::conditions::value_ilike::ValueILike;
use crate::select::conditions::value_in::ValueIn;
use crate::select::expression::Expression;
use crate::select::Select;

pub struct DefaultCharacterRepository<'a> {
  client: &'a Client,
  default_language: DbLanguage,
  image_repository: Arc<dyn ImageRepository + 'a>,
}

impl<'a> DefaultCharacterRepository<'a> {
  pub fn new(
    client: &'a Client,
    language: Language,
    image_repository: Arc<dyn ImageRepository + 'a>,
  ) -> DefaultCharacterRepository<'a> {
    DefaultCharacterRepository {
      client,
      default_language: language.into(),
      image_repository,
    }
  }
}

#[async_trait]
impl CharacterRepository for DefaultCharacterRepository<'_> {
  async fn get(&self, language: Language, pagination: Pagination) -> Result<ItemsTotal<Character>, Box<dyn Error>> {
    let language = DbLanguage::from(language);

    let total = Select::new::<DbCharacter>()
      .transform(|x| self.character_joins(x, &language))
      .query_count(self.client)
      .await? as usize;

    let characters = character_select_columns()
      .transform(|x| self.character_joins(x, &language))
      .pagination(pagination)
      .query(self.client)
      .await?;

    let characters = self.to_entities(characters).await?;

    Ok(ItemsTotal {
      items: characters,
      total,
    })
  }

  async fn get_by_id(&self, id: u32, language: Language) -> Result<Option<Character>, Box<dyn Error>> {
    let id = id as i32;
    let language = DbLanguage::from(language);
    let character = character_select_columns()
      .transform(|x| self.character_joins(x, &language))
      .where_expression(Expression::new(ValueEqual::new(("character", "id"), id)))
      .get_single(self.client)
      .await?;

    let fk_image = character.as_ref().and_then(|x| x.0.fk_image);

    let image = match fk_image {
      None => None,
      Some(x) => Some(self.image_repository.get_by_id(x as u32).await?.unwrap()),
    };

    Ok(character.map(|x| to_entity(x, image)))
  }

  async fn get_by_ids(&self, ids: &[u32], language: Language) -> Result<Vec<Character>, Box<dyn Error>> {
    let language = DbLanguage::from(language);
    let ids = to_i32(ids);

    let characters = character_select_columns()
      .transform(|x| self.character_joins(x, &language))
      .where_expression(Expression::new(ValueIn::new(("character", "id"), &ids)))
      .query(self.client)
      .await?;

    let characters = self.to_entities(characters).await?;

    Ok(characters)
  }

  async fn get_by_name(
    &self,
    name: &str,
    language: Language,
    pagination: Pagination,
  ) -> Result<ItemsTotal<Character>, Box<dyn Error>> {
    let language = DbLanguage::from(language);
    let name = format!("%{name}%");

    let total = Select::new::<DbCharacter>()
      .transform(|x| self.character_joins(x, &language))
      .query_count(self.client)
      .await? as usize;

    let characters = character_select_columns()
      .transform(|x| self.character_joins(x, &language))
      .where_expression(
        Expression::new(ValueILike::new(("character_translation", "name"), &name)).or(Expression::new(
          ValueILike::new(("character_translation_fallback", "name"), &name),
        )),
      )
      .pagination(pagination)
      .query(self.client)
      .await?;

    let characters = self.to_entities(characters).await?;

    Ok(ItemsTotal {
      items: characters,
      total,
    })
  }

  async fn filter_existing(&self, characters: &[u32]) -> Result<Vec<u32>, Box<dyn Error>> {
    let characters = to_i32(characters);

    let count = Select::new::<DbCharacter>()
      .column::<i32>(DbCharacter::TABLE_NAME, "id")
      .where_expression(Expression::new(ValueIn::new(
        (DbCharacter::TABLE_NAME, "id"),
        &characters,
      )))
      .query(self.client)
      .await?
      .into_iter()
      .map(|x| x.0 as u32)
      .collect();
    Ok(count)
  }
}

impl<'a> DefaultCharacterRepository<'a> {
  fn character_joins<T: FromRow<DbType = T> + CombinedType>(
    &'a self,
    select: Select<'a, T>,
    language: &'a DbLanguage,
  ) -> Select<'a, T> {
    select
      .left_join::<DbCharacterTranslation>(
        Some("character_translation"),
        Expression::value_equal("character_translation", "language", language).and(Expression::new(ColumnEqual::new(
          ("character_translation", "fktranslation"),
          ("character", "id"),
        ))),
      )
      .left_join::<DbCharacterTranslation>(
        Some("character_translation_fallback"),
        Expression::value_equal("character_translation_fallback", "language", self.default_language)
          .and(Expression::new(ColumnEqual::new(
            ("character_translation_fallback", "fktranslation"),
            ("character", "id"),
          )))
          .and(Expression::new(ColumnNull::new((
            "character_translation",
            "fktranslation",
          )))),
      )
  }
  async fn to_entities(&self, items: Vec<CharacterColumns>) -> Result<Vec<Character>, Box<dyn Error>> {
    if items.is_empty() {
      return Ok(vec![]);
    }

    let image_ids = image_ids(&items);
    let mut images = match image_ids.is_empty() {
      true => vec![],
      false => self.image_repository.get_by_ids(&image_ids).await?,
    };

    Ok(
      items
        .into_iter()
        .map(|x| {
          let fk_image = x.0.fk_image.map(|x| x as u32);
          let image = get_image(fk_image, &mut images);
          to_entity(x, image)
        })
        .collect(),
    )
  }
}

fn to_entity(
  character: (
    DbCharacter,
    Option<DbCharacterTranslation>,
    Option<DbCharacterTranslation>,
  ),
  image: Option<Image>,
) -> Character {
  character.0.to_entity(fallback_unwrap(character.1, character.2), image)
}

fn character_select_columns<'a>() -> Select<'a, CharacterColumns> {
  Select::new::<DbCharacter>()
    .columns::<DbCharacter>("character")
    .columns::<Option<DbCharacterTranslation>>("character_translation")
    .columns::<Option<DbCharacterTranslation>>("character_translation_fallback")
}

fn get_image(fk_image: Option<u32>, images: &mut Vec<Image>) -> Option<Image> {
  let fk_image = fk_image?;
  let index = images.iter().position(|x| x.id == fk_image);
  index.map(|index| images.swap_remove(index))
}

fn image_ids(items: &[CharacterColumns]) -> Vec<u32> {
  items.iter().filter_map(|x| x.0.fk_image.map(|x| x as u32)).collect()
}

type CharacterColumns = (
  DbCharacter,
  Option<DbCharacterTranslation>,
  Option<DbCharacterTranslation>,
);
