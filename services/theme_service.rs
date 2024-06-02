use async_trait::async_trait;

use domain::entities::theme::Theme;
use domain::enums::language::Language;
use domain::items_total::ItemsTotal;
use domain::pagination::Pagination;

use crate::traits::service_error::ServiceError;

#[async_trait]
pub trait ThemeService: Send + Sync {
  async fn get(&self, language: Language, pagination: Pagination) -> Result<ItemsTotal<Theme>, ServiceError>;
  async fn get_by_id(&self, id: u32, language: Language) -> Result<Option<Theme>, ServiceError>;
  async fn get_by_ids(&self, ids: &[i32], language: Language) -> Result<Vec<Theme>, ServiceError>;
  async fn get_by_name(&self, name: &str, language: Language,pagination: Pagination) -> Result<ItemsTotal<Theme>, ServiceError>;
}
