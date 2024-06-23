use std::sync::Arc;

use async_trait::async_trait;
use domain::entities::book::book_involved::InvolvedId;

use repositories::book_repository::book_involved_repository::BookInvolvedRepository;
use repositories::book_repository::book_involved_repository::mut_book_involved_repository::MutBookInvolvedRepository;
use repositories::book_repository::BookRepository;
use repositories::person_repository::PersonRepository;
use repositories::role_repository::RoleRepository;
use services::book_service::book_involved_service::mut_book_involved_service::MutBookInvolvedService;
use services::traits::service_error::{ClientError, ServiceError};

use crate::services::map_server_error;

pub struct DefaultMutBookInvolvedService<'a> {
  book_repository: Arc<dyn BookRepository + 'a>,
  book_involved_repository: Arc<dyn BookInvolvedRepository + 'a>,
  mut_book_involved_repository: Arc<dyn MutBookInvolvedRepository + 'a>,
  person_repository: Arc<dyn PersonRepository + 'a>,
  role_repository: Arc<dyn RoleRepository + 'a>,
}

impl<'a> DefaultMutBookInvolvedService<'a> {
  pub fn new(book_repository: Arc<dyn BookRepository + 'a>,
             book_involved_repository: Arc<dyn BookInvolvedRepository + 'a>,
             mut_book_involved_repository: Arc<dyn MutBookInvolvedRepository + 'a>,
             person_repository: Arc<dyn PersonRepository + 'a>,
             role_repository: Arc<dyn RoleRepository + 'a>, ) -> DefaultMutBookInvolvedService<'a> {
    DefaultMutBookInvolvedService { book_repository, book_involved_repository, mut_book_involved_repository, person_repository, role_repository }
  }
}

#[async_trait]
impl<'a> MutBookInvolvedService for DefaultMutBookInvolvedService<'a> {
  async fn add(&self, book_id: u32, involved: &[InvolvedId]) -> Result<(), ServiceError> {
    self.validate_add(book_id, involved).await?;
    self.mut_book_involved_repository.add(book_id, involved).await.map_err(map_server_error)
  }

  async fn remove(&self, book_id: u32, involved: &[InvolvedId]) -> Result<(), ServiceError> {
    self.validate_remove(book_id, involved).await?;
    self.mut_book_involved_repository.remove(book_id, involved).await.map_err(map_server_error)
  }
}

impl DefaultMutBookInvolvedService<'_> {
  async fn validate_add(&self, book_id: u32, involved: &[InvolvedId]) -> Result<(), ServiceError> {
    self.validate(book_id, involved).await?;
    let existing = self.book_involved_repository.filter_existing(book_id, involved).await.map_err(map_server_error)?;
    if !existing.is_empty() {
      return Err(ServiceError::ClientError(ClientError {
        title: "Invalid involved".to_string(),
        description: Some(format!("The following people with roles already have an association: [{}]", existing.into_iter().map(|x| x.to_string()).collect::<Vec<String>>().join(","))),
      }));
    };
    let people: Vec<u32> = involved.iter().map(|x| x.person_id).collect();
    let existing_people = self.person_repository
      .filter_existing(&people)
      .await.map_err(map_server_error)?;

    if existing_people.len() != involved.len() {
      let non_existent_people: Vec<String> = filter_non_existent(&people, &existing_people).into_iter().map(|x| x.to_string()).collect();
      return Err(ServiceError::ClientError(ClientError {
        title: "Invalid people".to_string(),
        description: Some(format!("The following people do not exist: [{}]", non_existent_people.join(","))),
      }));
    };
    let roles: Vec<u32> = involved.iter().map(|x| x.person_id).collect();
    let existing_roles = self.role_repository
      .filter_existing(&roles)
      .await.map_err(map_server_error)?;
    if existing_roles.len() != involved.len() {
      let non_existent_roles: Vec<String> = filter_non_existent(&roles, &existing_roles).into_iter().map(|x| x.to_string()).collect();
      return Err(ServiceError::ClientError(ClientError {
        title: "Invalid roles".to_string(),
        description: Some(format!("The following roles do not exist: [{}]", non_existent_roles.join(","))),
      }));
    };

    Ok(())
  }
  async fn validate_remove(&self, book_id: u32, involved: &[InvolvedId]) -> Result<(), ServiceError> {
    self.validate(book_id, involved).await?;
    let existing = self.book_involved_repository.filter_existing(book_id, involved).await.map_err(map_server_error)?;
    if existing.len() != involved.len() {
      let non_existent_involved: Vec<String> = filter_involved_non_existent(involved, &existing).into_iter().map(|x| x.to_string()).collect();
      return Err(ServiceError::ClientError(ClientError {
        title: "Invalid involved".to_string(),
        description: Some(format!("The following people with roles do not have an association: [{}]", non_existent_involved.join(","))),
      }));
    };

    Ok(())
  }
  async fn validate(&self, book_id: u32, involved: &[InvolvedId]) -> Result<(), ServiceError> {
    let ids = self.book_repository.filter_existing(&[book_id]).await.map_err(map_server_error)?;
    if ids.is_empty() {
      return Err(ServiceError::ClientError(ClientError {
        title: format!("Book with the id {book_id} does not exist"),
        description: None,
      }));
    }
    if involved.is_empty() {
      return Err(ServiceError::ClientError(ClientError {
        title: "No involved provided".to_string(),
        description: None,
      }));
    }
    Ok(())
  }
}

fn filter_non_existent(items: &[u32], existing: &[u32]) -> Vec<u32> {
  items.iter().filter_map(|x|
    existing.iter()
      .find(|y| **y == *x)
      .map(|_| None)
      .unwrap_or(Some(*x))
  ).collect()
}
fn filter_involved_non_existent(items: &[InvolvedId], existing: &[InvolvedId]) -> Vec<InvolvedId> {
  items.iter().filter_map(|x|
    existing.iter()
      .find(|y| y.role_id == x.role_id && y.person_id == x.person_id)
      .map(|_| None)
      .unwrap_or(Some(x.clone()))
  ).collect()
}
