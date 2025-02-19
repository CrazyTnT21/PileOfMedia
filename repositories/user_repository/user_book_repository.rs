use std::collections::HashMap;
use std::error::Error;

use async_trait::async_trait;
use domain::entities::user::user_book::UserBook;
use domain::enums::language::Language;

pub mod mut_user_book_repository;

#[async_trait]
pub trait UserBookRepository: Send + Sync {
  //TODO Return Option if user doesn't exist
  async fn get_by_book_id(
    &self,
    user_id: u32,
    book_id: u32,
    languages: &[Language],
  ) -> Result<Option<UserBook>, Box<dyn Error>>;
  //TODO Return Option if user doesn't exist
  async fn get_by_book_ids(
    &self,
    user_id: u32,
    book_ids: &[u32],
    languages: &[Language],
  ) -> Result<Vec<UserBook>, Box<dyn Error>>;
  //TODO Return Option if user doesn't exist
  async fn get_by_user_id(&self, user_id: u32, languages: &[Language]) -> Result<Vec<UserBook>, Box<dyn Error>>;
  async fn get_by_user_ids(
    &self,
    user_ids: &[u32],
    languages: &[Language],
  ) -> Result<HashMap<u32, Vec<UserBook>>, Box<dyn Error>>;
  async fn filter_existing(&self, ids: HashMap<u32, Vec<u32>>) -> Result<HashMap<u32, Vec<u32>>, Box<dyn Error>>;
}
