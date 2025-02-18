use async_trait::async_trait;
use domain::entities::user::create_user_book::CreateUserBook;
use domain::entities::user::user_book::UserBook;
use domain::enums::language::Language;
use std::error::Error;

#[async_trait]
pub trait MutUserBookRepository: Send + Sync {
  async fn add(&self, user_id: u32, book: CreateUserBook, languages: &[Language]) -> Result<UserBook, Box<dyn Error>>;
  async fn remove(&self, user_id: u32, book_ids: &[u32]) -> Result<(), Box<dyn Error>>;

  async fn remove_all(&self, user_ids: &[u32]) -> Result<(), Box<dyn Error>>;
}
