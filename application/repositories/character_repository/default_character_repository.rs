use async_trait::async_trait;
use domain::available_translations::AvailableTranslations;
use std::collections::HashMap;
use std::error::Error;
use std::hash::Hash;
use std::sync::Arc;
use tokio_postgres::Client;

use crate::convert_to_sql::to_i32;
use crate::enums::db_language::DbLanguage;
use crate::schemas::db_character::DbCharacter;
use crate::schemas::db_character_translation::DbCharacterTranslation;
use crate::select::combined_tuple::CombinedType;
use crate::select::conditions::value_equal::ValueEqual;
use crate::select::conditions::value_ilike::ValueILike;
use crate::select::conditions::value_in::ValueIn;
use crate::select::expression::Expression;
use crate::select::Select;
use domain::entities::character::character_translation::CharacterTranslation;
use domain::entities::character::Character;
use domain::entities::image::Image;
use domain::enums::language::Language;
use domain::items_total::ItemsTotal;
use domain::pagination::Pagination;
use from_row::{FromRow, Table};
use repositories::character_repository::CharacterRepository;
use repositories::image_repository::ImageRepository;

pub struct DefaultCharacterRepository<'a> {
  client: &'a Client,
  image_repository: Arc<dyn ImageRepository + 'a>,
}

impl<'a> DefaultCharacterRepository<'a> {
  pub const fn new(
    client: &'a Client,
    image_repository: Arc<dyn ImageRepository + 'a>,
  ) -> DefaultCharacterRepository<'a> {
    DefaultCharacterRepository {
      client,
      image_repository,
    }
  }
}

#[async_trait]
impl CharacterRepository for DefaultCharacterRepository<'_> {
  async fn get(&self, languages: &[Language], pagination: Pagination) -> Result<ItemsTotal<Character>, Box<dyn Error>> {
    let db_languages: Vec<DbLanguage> = languages.iter().map(|x| (*x).into()).collect();
    let total = Select::new::<DbCharacter>()
      .transform(inner_join_translation)
      .query_count(self.client)
      .await? as usize;

    let characters = Select::new::<DbCharacter>()
      .distinct_on(DbCharacter::TABLE_NAME, "id")
      .columns_table::<DbCharacter>()
      .transform(inner_join_translation)
      .pagination(pagination)
      .query_destruct(self.client)
      .await?;
    let character_ids: Vec<i32> = characters.iter().map(|x| x.id).collect();

    let mut translations = character_translation_select(&character_ids, &db_languages)
      .query_destruct(self.client)
      .await?;

    let no_translations: Vec<i32> = no_translation_ids(&characters, &translations);

    let mut extra_translations = Select::new::<DbCharacterTranslation>()
      .distinct_on(DbCharacterTranslation::TABLE_NAME, "fktranslation")
      .columns::<DbCharacterTranslation>(DbCharacterTranslation::TABLE_NAME)
      .where_expression(fk_translation_in_ids(&no_translations))
      .query_destruct(self.client)
      .await?;

    translations.append(&mut extra_translations);

    let translations: Vec<(Language, i32, CharacterTranslation)> = translations
      .into_iter()
      .map(|x| (x.language.into(), x.fk_translation, x.to_entity()))
      .collect();

    let translations = map_translation(&characters, translations);

    let image_ids: Vec<i32> = characters.iter().filter_map(|x| x.fk_image).collect();
    let image_ids: Vec<u32> = to_u32(image_ids);
    let images = self.image_repository.get_by_ids(&image_ids).await?;

    let available = self.available_languages(&character_ids).await?;
    let characters = to_entities(characters, available, translations, images);
    Ok(ItemsTotal {
      items: characters,
      total,
    })
  }

  async fn get_by_id(&self, id: u32, languages: &[Language]) -> Result<Option<Character>, Box<dyn Error>> {
    let id = id as i32;
    let db_languages: Vec<DbLanguage> = languages.iter().map(|x| (*x).into()).collect();

    let characters = Select::new::<DbCharacter>()
      .columns_table::<DbCharacter>()
      .distinct_on(DbCharacter::TABLE_NAME, "id")
      .inner_join::<DbCharacterTranslation>(
        None,
        Expression::new(ValueEqual::new((DbCharacter::TABLE_NAME, "id"), id)).and(character_id_equal_fk_translation()),
      )
      .get_single(self.client)
      .await?;
    let Some(item) = characters else {
      return Ok(None);
    };
    let translations: Vec<(Language, CharacterTranslation)> = Select::new::<DbCharacterTranslation>()
      .columns::<DbCharacterTranslation>(DbCharacterTranslation::TABLE_NAME)
      .where_expression(
        Expression::value_equal(DbCharacterTranslation::TABLE_NAME, "fktranslation", item.0.id)
          .and(in_languages(&db_languages)),
      )
      .query(self.client)
      .await?
      .into_iter()
      .map(|x| (x.0.language.into(), x.0.to_entity()))
      .collect();
    let mut available = self.available_languages(&[id]).await?;
    let image = match item.0.fk_image {
      None => None,
      Some(fk_image) => self.image_repository.get_by_id(fk_image as u32).await?,
    };
    let item = item.0.to_entity(
      AvailableTranslations {
        available_languages: available.remove(&id).unwrap(),
        translations: HashMap::from_iter(translations),
      },
      image,
    );
    Ok(Some(item))
  }

  async fn get_by_ids(&self, ids: &[u32], languages: &[Language]) -> Result<Vec<Character>, Box<dyn Error>> {
    let ids = to_i32(ids);
    let db_languages: Vec<DbLanguage> = languages.iter().map(|x| (*x).into()).collect();

    let characters = Select::new::<DbCharacter>()
      .columns_table::<DbCharacter>()
      .distinct_on(DbCharacter::TABLE_NAME, "id")
      .transform(inner_join_translation)
      .where_expression(Expression::new(ValueIn::new((DbCharacter::TABLE_NAME, "id"), &ids)))
      .query_destruct(self.client)
      .await?;

    if characters.is_empty() {
      return Ok(vec![]);
    }
    let character_ids: Vec<i32> = characters.iter().map(|x| x.id).collect();

    let mut translations = character_translation_select(&character_ids, &db_languages)
      .query_destruct(self.client)
      .await?;

    let no_translations: Vec<i32> = no_translation_ids(&characters, &translations);

    let mut extra_translations = Select::new::<DbCharacterTranslation>()
      .distinct_on(DbCharacterTranslation::TABLE_NAME, "fktranslation")
      .columns::<DbCharacterTranslation>(DbCharacterTranslation::TABLE_NAME)
      .where_expression(fk_translation_in_ids(&no_translations))
      .query_destruct(self.client)
      .await?;

    translations.append(&mut extra_translations);

    let translations: Vec<(Language, i32, CharacterTranslation)> = translations
      .into_iter()
      .map(|x| (x.language.into(), x.fk_translation, x.to_entity()))
      .collect();

    let image_ids: Vec<i32> = characters.iter().filter_map(|x| x.fk_image).collect();
    let image_ids: Vec<u32> = to_u32(image_ids);
    let images = self.image_repository.get_by_ids(&image_ids).await?;

    let translations = map_translation(&characters, translations);
    let available = self.available_languages(&character_ids).await?;
    let characters = to_entities(characters, available, translations, images);
    Ok(characters)
  }

  async fn get_by_name(
    &self,
    name: &str,
    languages: &[Language],
    pagination: Pagination,
  ) -> Result<ItemsTotal<Character>, Box<dyn Error>> {
    let name = format!("%{name}%");

    let db_languages: Vec<DbLanguage> = languages.iter().map(|x| (*x).into()).collect();
    let total = Select::new::<DbCharacter>()
      .transform(|x| inner_join_translation_on_name(x, &name, &db_languages))
      .query_count(self.client)
      .await? as usize;

    let characters = Select::new::<DbCharacter>()
      .columns_table::<DbCharacter>()
      .distinct_on(DbCharacter::TABLE_NAME, "id")
      .transform(|x| inner_join_translation_on_name(x, &name, &db_languages))
      .pagination(pagination)
      .query_destruct(self.client)
      .await?;
    let character_ids: Vec<i32> = characters.iter().map(|x| x.id).collect();

    let mut translations = character_translation_select(&character_ids, &db_languages)
      .where_expression(character_translation_with_name(&name))
      .query_destruct(self.client)
      .await?;

    let no_translations: Vec<i32> = no_translation_ids(&characters, &translations);

    let mut extra_translations = Select::new::<DbCharacterTranslation>()
      .distinct_on(DbCharacterTranslation::TABLE_NAME, "fktranslation")
      .columns::<DbCharacterTranslation>(DbCharacterTranslation::TABLE_NAME)
      .where_expression(fk_translation_in_ids(&no_translations))
      .query_destruct(self.client)
      .await?;

    translations.append(&mut extra_translations);

    let translations: Vec<(Language, i32, CharacterTranslation)> = translations
      .into_iter()
      .map(|x| (x.language.into(), x.fk_translation, x.to_entity()))
      .collect();

    let image_ids: Vec<i32> = characters.iter().filter_map(|x| x.fk_image).collect();
    let image_ids: Vec<u32> = to_u32(image_ids);
    let images = self.image_repository.get_by_ids(&image_ids).await?;

    let translations = map_translation(&characters, translations);
    let available = self.available_languages(&character_ids).await?;
    let characters = to_entities(characters, available, translations, images);
    Ok(ItemsTotal {
      items: characters,
      total,
    })
  }

  async fn filter_existing(&self, characters: &[u32]) -> Result<Vec<u32>, Box<dyn Error>> {
    let characters = to_i32(characters);

    let count = Select::new::<DbCharacter>()
      .column::<i32>(DbCharacter::TABLE_NAME, "id")
      .where_expression(id_in_ids(&characters))
      .query(self.client)
      .await?
      .into_iter()
      .map(|x| x.0 as u32)
      .collect();
    Ok(count)
  }
}
impl DefaultCharacterRepository<'_> {
  async fn available_languages(&self, ids: &[i32]) -> Result<HashMap<i32, Vec<Language>>, Box<dyn Error>> {
    let available_translations = Select::new::<DbCharacterTranslation>()
      .column::<i32>(DbCharacterTranslation::TABLE_NAME, "fktranslation")
      .column::<DbLanguage>(DbCharacterTranslation::TABLE_NAME, "language")
      .where_expression(Expression::new(ValueIn::new(
        (DbCharacterTranslation::TABLE_NAME, "fktranslation"),
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
fn character_translation_select<'a>(
  character_ids: &'a [i32],
  db_languages: &'a [DbLanguage],
) -> Select<'a, (DbCharacterTranslation,)> {
  Select::new::<DbCharacterTranslation>()
    .columns::<DbCharacterTranslation>(DbCharacterTranslation::TABLE_NAME)
    .where_expression(Expression::new(ValueIn::new(
      (DbCharacterTranslation::TABLE_NAME, "fktranslation"),
      character_ids,
    )))
    .where_expression(in_languages(db_languages))
}
fn map_translation(
  characters: &[DbCharacter],
  translations: Vec<(Language, i32, CharacterTranslation)>,
) -> HashMap<i32, Vec<(Language, CharacterTranslation)>> {
  let mut new_translations: HashMap<i32, Vec<(Language, CharacterTranslation)>> = HashMap::new();
  for character in characters {
    new_translations.insert(character.id, vec![]);
  }
  for (language, id, translation) in translations {
    let result = new_translations.get_mut(&id).unwrap();
    result.push((language, translation));
  }
  new_translations
}
fn character_id_equal_fk_translation<'a>() -> Expression<'a> {
  Expression::column_equal(
    (DbCharacter::TABLE_NAME, "id"),
    (DbCharacterTranslation::TABLE_NAME, "fktranslation"),
  )
}
fn character_translation_with_name(name: &String) -> Expression {
  Expression::new(ValueILike::new((DbCharacterTranslation::TABLE_NAME, "name"), name))
}
fn inner_join_translation_on_name<'a, T: FromRow<DbType = T> + CombinedType>(
  select: Select<'a, T>,
  name: &'a String,
  db_languages: &'a [DbLanguage],
) -> Select<'a, T> {
  select.inner_join::<DbCharacterTranslation>(
    None,
    Expression::value_i_like((DbCharacterTranslation::TABLE_NAME, "name"), name)
      .and(in_languages(db_languages))
      .and(character_id_equal_fk_translation()),
  )
}
fn to_entities(
  characters: Vec<DbCharacter>,
  mut available: HashMap<i32, Vec<Language>>,
  mut translations: HashMap<i32, Vec<(Language, CharacterTranslation)>>,
  mut images: Vec<Image>,
) -> Vec<Character> {
  characters
    .into_iter()
    .map(|character| {
      let id = character.id;
      let image_id = character.fk_image;
      let image = image_id.and_then(|x| images.iter().position(|y| y.id == x as u32).map(|x| images.remove(x)));
      character.to_entity(
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
  select.inner_join::<DbCharacterTranslation>(None, character_id_equal_fk_translation())
}
fn in_languages(languages: &[DbLanguage]) -> Expression {
  Expression::new(ValueIn::new(
    (DbCharacterTranslation::TABLE_NAME, "language"),
    languages,
  ))
}
fn to_u32(values: Vec<i32>) -> Vec<u32> {
  values.into_iter().map(|x| x as u32).collect()
}
fn fk_translation_in_ids(ids: &[i32]) -> Expression {
  Expression::new(ValueIn::new((DbCharacterTranslation::TABLE_NAME, "fktranslation"), ids))
}
fn id_in_ids(ids: &[i32]) -> Expression {
  Expression::new(ValueIn::new((DbCharacter::TABLE_NAME, "id"), ids))
}
fn no_translation_ids(character_ids: &[DbCharacter], translations: &[DbCharacterTranslation]) -> Vec<i32> {
  character_ids
    .iter()
    .filter_map(|x| {
      translations
        .iter()
        .find(|y| y.fk_translation == x.id)
        .map_or(Some(x.id), |_| None)
    })
    .collect()
}
