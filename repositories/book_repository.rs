use async_trait::async_trait;
use domain::entities::book::Book;
use domain::enums::language::Language;
use domain::items_total::ItemsTotal;
use domain::pagination::Pagination;
use std::error::Error;

pub mod book_character_repository;
pub mod book_genre_repository;
pub mod book_involved_repository;
pub mod book_theme_repository;
pub mod mut_book_repository;

#[async_trait]
pub trait BookRepository: Send + Sync {
  async fn get(&self, language: Language, pagination: Pagination) -> Result<ItemsTotal<Book>, Box<dyn Error>>;
  async fn get_by_id(&self, id: u32, language: Language) -> Result<Option<Book>, Box<dyn Error>>;
  async fn get_by_title(
    &self,
    title: &str,
    language: Language,
    pagination: Pagination,
  ) -> Result<ItemsTotal<Book>, Box<dyn Error>>;
  async fn get_by_ids(&self, ids: &[u32], language: Language) -> Result<Vec<Book>, Box<dyn Error>>;

  async fn filter_existing(&self, books: &[u32]) -> Result<Vec<u32>, Box<dyn Error>>;
}
