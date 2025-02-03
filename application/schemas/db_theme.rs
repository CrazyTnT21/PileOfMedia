use domain::available_translations::AvailableTranslations;
use domain::entities::theme::theme_translation::ThemeTranslation;
use domain::entities::theme::Theme;
use from_row::FromRow;
use tokio_postgres::Row;

#[derive(FromRow, Debug)]
#[rename = "theme"]
pub struct DbTheme {
  pub id: i32,
}

impl DbTheme {
  pub const fn to_entity(self, translations: AvailableTranslations<ThemeTranslation>) -> Theme {
    Theme {
      id: self.id as u32,
      translations,
    }
  }
}
