use crate::enums::db_language::DbLanguage;
use domain::entities::person::person_translation::PersonTranslation;
use from_row::FromRow;
use tokio_postgres::Row;

#[derive(FromRow, Debug)]
#[rename = "person_translation"]
pub struct DbPersonTranslation {
  pub description: Option<String>,
  pub translation_id: i32,
  pub language: DbLanguage,
}

impl DbPersonTranslation {
  pub fn to_entity(self) -> PersonTranslation {
    PersonTranslation {
      description: self.description,
    }
  }
}
