use domain::available_translations::AvailableTranslations;
use domain::entities::role::Role;
use domain::entities::role::role_translation::RoleTranslation;
use from_row::FromRow;
use tokio_postgres::Row;

#[derive(FromRow, Debug)]
#[rename = "role"]
pub struct DbRole {
  pub id: i32,
}

impl DbRole {
  pub const fn to_entity(self, translations: AvailableTranslations<RoleTranslation>) -> Role {
    Role {
      id: self.id as u32,
      translations,
    }
  }
}
