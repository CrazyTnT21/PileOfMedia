use tokio_postgres::Row;
use domain::entities::role::Role;
use from_row::FromRow;
use crate::schemas::db_role_translation::DbRoleTranslation;

#[derive(FromRow, Debug)]
#[rename = "role"]
pub struct DbRole {
  pub id: i32
}

impl DbRole {
  pub fn to_entity(self, role_translation: DbRoleTranslation) -> Role {
    Role {
      id: self.id,
      name: role_translation.name
    }
  }
}
