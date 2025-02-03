use crate::enums::db_language::DbLanguage;
use domain::entities::person::person_translation::PersonTranslation;
use from_row::FromRow;
use tokio_postgres::Row;

#[derive(FromRow, Debug)]
#[rename = "persontranslation"]
pub struct DbPersonTranslation {
  pub description: Option<String>,
  #[rename = "fktranslation"]
  pub fk_translation: i32,
  pub language: DbLanguage,
}

impl DbPersonTranslation {
  pub fn to_entity(self) -> PersonTranslation {
    PersonTranslation {
      description: self.description,
    }
  }
}
