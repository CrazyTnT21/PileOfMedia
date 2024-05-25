use tokio_postgres::Row;

use domain::entities::theme::Theme;
use from_row::FromRow;

use crate::schemas::db_theme_translation::DbThemeTranslation;

#[derive(FromRow, Debug)]
#[rename = "theme"]
pub struct DbTheme {
  pub id: i32
}

impl DbTheme {
  pub fn to_entity(self, translation: DbThemeTranslation) -> Theme {
   Theme {
     id: self.id,
     name: translation.name
   }
  }
}
