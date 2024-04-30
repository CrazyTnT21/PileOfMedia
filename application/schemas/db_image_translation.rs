use from_row::FromRow;
use tokio_postgres::Row;
use crate::enums::image_extension::DbImageExtension;
use crate::enums::language::DbLanguage;

#[derive(FromRow, Debug)]
pub struct DbImageTranslation {
  #[rename = "fktranslation"]
  pub fk_translation: i32,
  pub language: DbLanguage,
  pub uri: String,
  pub width: i16,
  pub height: i16,
  pub extension: Option<DbImageExtension>,
}
