use std::sync::Arc;

use async_trait::async_trait;

use domain::entities::involved::InvolvedId;
use repositories::book_repository::BookRepository;
use repositories::book_repository::book_involved_repository::BookInvolvedRepository;
use repositories::book_repository::book_involved_repository::mut_book_involved_repository::MutBookInvolvedRepository;
use repositories::person_repository::PersonRepository;
use repositories::role_repository::RoleRepository;
use services::book_service::book_involved_service::mut_book_involved_service::{
  MutBookInvolvedService, MutBookInvolvedServiceError,
};
use services::traits::service_error::ServiceError;

pub struct DefaultMutBookInvolvedService<'a> {
  book_repository: Arc<dyn BookRepository + 'a>,
  book_involved_repository: Arc<dyn BookInvolvedRepository + 'a>,
  mut_book_involved_repository: Arc<dyn MutBookInvolvedRepository + 'a>,
  person_repository: Arc<dyn PersonRepository + 'a>,
  role_repository: Arc<dyn RoleRepository + 'a>,
}

impl<'a> DefaultMutBookInvolvedService<'a> {
  pub fn new(
    book_repository: Arc<dyn BookRepository + 'a>,
    book_involved_repository: Arc<dyn BookInvolvedRepository + 'a>,
    mut_book_involved_repository: Arc<dyn MutBookInvolvedRepository + 'a>,
    person_repository: Arc<dyn PersonRepository + 'a>,
    role_repository: Arc<dyn RoleRepository + 'a>,
  ) -> DefaultMutBookInvolvedService<'a> {
    DefaultMutBookInvolvedService {
      book_repository,
      book_involved_repository,
      mut_book_involved_repository,
      person_repository,
      role_repository,
    }
  }
}

#[async_trait]
impl MutBookInvolvedService for DefaultMutBookInvolvedService<'_> {
  async fn add(&self, book_id: u32, involved: &[InvolvedId]) -> Result<(), ServiceError<MutBookInvolvedServiceError>> {
    self.validate_add(book_id, involved).await?;
    Ok(self.mut_book_involved_repository.add(book_id, involved).await?)
  }

  async fn remove(
    &self,
    book_id: u32,
    involved: &[InvolvedId],
  ) -> Result<(), ServiceError<MutBookInvolvedServiceError>> {
    self.validate_remove(book_id, involved).await?;
    Ok(self.mut_book_involved_repository.remove(book_id, involved).await?)
  }
}

impl DefaultMutBookInvolvedService<'_> {
  async fn validate_add(
    &self,
    book_id: u32,
    involved: &[InvolvedId],
  ) -> Result<(), ServiceError<MutBookInvolvedServiceError>> {
    self.validate(book_id, involved).await?;
    let existing = self.book_involved_repository.filter_existing(book_id, involved).await?;
    if !existing.is_empty() {
      return Err(ServiceError::ClientError(
        MutBookInvolvedServiceError::AlreadyAssociated(existing),
      ));
    };
    let mut people: Vec<u32> = involved.iter().map(|x| x.person_id).collect();
    people.sort_unstable();
    people.dedup();
    let existing_people = self.person_repository.filter_existing(&people).await?;

    if existing_people.len() != involved.len() {
      let non_existent_people = filter_non_existent(&people, &existing_people);
      return Err(ServiceError::ClientError(
        MutBookInvolvedServiceError::NonExistentPeople(non_existent_people),
      ));
    };
    let mut roles: Vec<u32> = involved.iter().map(|x| x.role_id).collect();
    roles.sort_unstable();
    roles.dedup();
    let existing_roles = self.role_repository.filter_existing(&roles).await?;

    if existing_roles.len() != involved.len() {
      let non_existent_roles = filter_non_existent(&roles, &existing_roles);
      return Err(ServiceError::ClientError(
        MutBookInvolvedServiceError::NonExistentRoles(non_existent_roles),
      ));
    };

    Ok(())
  }
  async fn validate_remove(
    &self,
    book_id: u32,
    involved: &[InvolvedId],
  ) -> Result<(), ServiceError<MutBookInvolvedServiceError>> {
    self.validate(book_id, involved).await?;
    let existing = self.book_involved_repository.filter_existing(book_id, involved).await?;
    if existing.len() != involved.len() {
      let non_existent_involved = filter_involved_non_existent(involved, &existing);
      return Err(ServiceError::ClientError(
        MutBookInvolvedServiceError::NonExistentAssociation(non_existent_involved),
      ));
    };

    Ok(())
  }
  async fn validate(
    &self,
    book_id: u32,
    involved: &[InvolvedId],
  ) -> Result<(), ServiceError<MutBookInvolvedServiceError>> {
    let ids = self.book_repository.filter_existing(&[book_id]).await?;
    if ids.is_empty() {
      return Err(ServiceError::ClientError(MutBookInvolvedServiceError::NonExistentBook(
        book_id,
      )));
    }
    if involved.is_empty() {
      return Err(ServiceError::ClientError(
        MutBookInvolvedServiceError::NoInvolvedProvided,
      ));
    }
    Ok(())
  }
}

fn filter_non_existent(items: &[u32], existing: &[u32]) -> Vec<u32> {
  items
    .iter()
    .filter_map(|x| existing.iter().find(|y| **y == *x).map_or(Some(*x), |_| None))
    .collect()
}

fn filter_involved_non_existent(items: &[InvolvedId], existing: &[InvolvedId]) -> Vec<InvolvedId> {
  items
    .iter()
    .filter_map(|x| {
      existing
        .iter()
        .find(|y| y.role_id == x.role_id && y.person_id == x.person_id)
        .map_or_else(|| Some(x.clone()), |_| None)
    })
    .collect()
}
