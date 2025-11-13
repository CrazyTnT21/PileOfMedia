use crate::enums::db_language::DbLanguage;
use domain::entities::character::character_translation::CharacterTranslation;
use from_row::FromRow;
use tokio_postgres::Row;

#[derive(FromRow, Debug)]
#[rename = "character_translation"]
pub struct DbCharacterTranslation {
  pub name: String,
  pub first_name: Option<String>,
  pub last_name: Option<String>,
  pub description: Option<String>,
  pub translation_id: i32,
  pub language: DbLanguage,
}
impl DbCharacterTranslation {
  pub fn to_entity(self) -> CharacterTranslation {
    CharacterTranslation {
      name: self.name,
      first_name: self.first_name,
      last_name: self.last_name,
      description: self.description,
    }
  }
}
