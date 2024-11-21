pub mod mut_book_involved_repository;

use std::error::Error;

use async_trait::async_trait;

use domain::entities::book::book_involved::{BookInvolved, InvolvedId};
use domain::enums::language::Language;
use domain::items_total::ItemsTotal;
use domain::pagination::Pagination;

#[async_trait]
pub trait BookInvolvedRepository: Send + Sync {
  async fn get(
    &self,
    book_id: u32,
    language: Language,
    pagination: Pagination,
  ) -> Result<ItemsTotal<BookInvolved>, Box<dyn Error>>;
  async fn filter_existing(&self, book_id: u32, involved: &[InvolvedId]) -> Result<Vec<InvolvedId>, Box<dyn Error>>;
}
