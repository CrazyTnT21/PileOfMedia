use crate::enums::db_language::DbLanguage;
use domain::entities::role::role_translation::RoleTranslation;
use from_row::FromRow;
use tokio_postgres::Row;

#[derive(FromRow, Debug)]
#[rename = "roletranslation"]
pub struct DbRoleTranslation {
  pub name: String,
  #[rename = "fktranslation"]
  pub fk_translation: i32,
  pub language: DbLanguage,
}

impl DbRoleTranslation {
  pub fn to_entity(self) -> RoleTranslation {
    RoleTranslation { name: self.name }
  }
}
