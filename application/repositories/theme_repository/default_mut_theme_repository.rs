use std::error::Error;
use std::sync::Arc;

use async_trait::async_trait;
use tokio_postgres::Transaction;

use domain::entities::theme::create_partial_theme::CreatePartialTheme;
use domain::entities::theme::Theme;
use domain::enums::language::Language;
use from_row::Table;
use repositories::theme_repository::mut_theme_repository::MutThemeRepository;
use repositories::theme_repository::ThemeRepository;

use crate::convert_to_sql::to_i32;
use crate::delete::Delete;
use crate::enums::db_language::DbLanguage;
use crate::insert::Insert;
use crate::schemas::db_theme::DbTheme;
use crate::schemas::db_theme_translation::DbThemeTranslation;
use crate::select::conditions::value_in::ValueIn;
use crate::select::expression::Expression;

pub struct DefaultMutThemeRepository<'a> {
  transaction: &'a Transaction<'a>,
  default_language: Language,
  theme_repository: Arc<dyn ThemeRepository + 'a>,
}

impl<'a> DefaultMutThemeRepository<'a> {
  pub fn new(
    transaction: &'a Transaction<'a>,
    default_language: Language,
    theme_repository: Arc<dyn ThemeRepository + 'a>,
  ) -> DefaultMutThemeRepository<'a> {
    DefaultMutThemeRepository {
      transaction,
      default_language,
      theme_repository,
    }
  }
}

#[async_trait]
impl MutThemeRepository for DefaultMutThemeRepository<'_> {
  async fn create(&self, item: CreatePartialTheme) -> Result<Theme, Box<dyn Error>> {
    let id = self.insert_theme(&item).await? as u32;
    self.insert_translation(&item, id).await?;

    let theme = self
      .theme_repository
      .get_by_id(id, self.default_language)
      .await?
      .expect("Theme was just created");
    Ok(theme)
  }

  async fn delete(&self, ids: &[u32]) -> Result<(), Box<dyn Error>> {
    let ids = to_i32(ids);

    Delete::new::<DbThemeTranslation>(Expression::new(ValueIn::new(
      (DbThemeTranslation::TABLE_NAME, "fktranslation"),
      &ids,
    )))
    .execute_transaction(self.transaction)
    .await?;

    Delete::new::<DbTheme>(Expression::new(ValueIn::new((DbTheme::TABLE_NAME, "id"), &ids)))
      .execute_transaction(self.transaction)
      .await?;
    Ok(())
  }
}

impl DefaultMutThemeRepository<'_> {
  async fn insert_theme(&self, _item: &CreatePartialTheme) -> Result<i32, Box<dyn Error>> {
    let id = Insert::new::<DbTheme>([])
      .returning_transaction("id", self.transaction)
      .await?;
    Ok(id)
  }
  async fn insert_translation(&self, item: &CreatePartialTheme, id: u32) -> Result<(), Box<dyn Error>> {
    let id = id as i32;
    let translations: Vec<(&String, DbLanguage)> = item
      .translations
      .iter()
      .map(|x| (&x.1.name, DbLanguage::from(*x.0)))
      .collect();

    let mut insert = Insert::new::<DbThemeTranslation>(["name", "fktranslation", "language"]);
    for (title, language) in &translations {
      insert.values_ref([*title, &id, language]);
    }
    insert.execute_transaction(self.transaction).await?;
    Ok(())
  }
}
