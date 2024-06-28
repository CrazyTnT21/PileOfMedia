use std::error::Error;

use async_trait::async_trait;

use domain::entities::person::create_partial_person::CreatePartialPerson;
use domain::entities::person::Person;

#[async_trait]
pub trait MutPersonRepository: Send + Sync {
  async fn create(&self, item: CreatePartialPerson) -> Result<Person, Box<dyn Error>>;
  async fn delete(&self, ids: &[u32]) -> Result<(), Box<dyn Error>>;
}
