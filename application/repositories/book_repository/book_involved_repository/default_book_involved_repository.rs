use std::collections::HashMap;
use std::error::Error;
use std::sync::Arc;

use async_trait::async_trait;
use tokio_postgres::Client;

use crate::convert_to_sql::to_i32;
use crate::schemas::db_book_involved::DbBookInvolved;
use crate::select::Select;
use crate::select::conditions::value_equal::ValueEqual;
use crate::select::conditions::value_in::ValueIn;
use crate::select::expression::Expression;
use domain::entities::book::book_involved::BookInvolved;
use domain::entities::involved::InvolvedId;
use domain::entities::person::Person;
use domain::entities::role::Role;
use domain::enums::language::Language;
use from_row::Table;
use repositories::book_repository::book_involved_repository::BookInvolvedRepository;
use repositories::person_repository::PersonRepository;
use repositories::role_repository::RoleRepository;

pub struct DefaultBookInvolvedRepository<'a> {
  client: &'a Client,
  person_repository: Arc<dyn PersonRepository + 'a>,
  role_repository: Arc<dyn RoleRepository + 'a>,
}

impl<'a> DefaultBookInvolvedRepository<'a> {
  pub fn new(
    client: &'a Client,
    person_repository: Arc<dyn PersonRepository + 'a>,
    role_repository: Arc<dyn RoleRepository + 'a>,
  ) -> DefaultBookInvolvedRepository<'a> {
    DefaultBookInvolvedRepository {
      client,
      person_repository,
      role_repository,
    }
  }
}

#[async_trait]
impl BookInvolvedRepository for DefaultBookInvolvedRepository<'_> {
  async fn get_by_id(&self, book_id: u32, languages: &[Language]) -> Result<Vec<BookInvolved>, Box<dyn Error>> {
    let book_id = book_id as i32;

    let involved = Select::new::<DbBookInvolved>()
      .columns::<DbBookInvolved>(DbBookInvolved::TABLE_NAME)
      .where_expression(book_id_equal_id(book_id))
      .query_destruct(self.client)
      .await?;

    if involved.is_empty() {
      return Ok(vec![]);
    }
    let mut person_ids: Vec<u32> = involved.iter().map(|x| x.person_id as u32).collect();
    person_ids.sort_unstable();
    person_ids.dedup();

    let mut role_ids: Vec<u32> = involved.iter().map(|x| x.role_id as u32).collect();
    role_ids.sort_unstable();
    role_ids.dedup();

    let people = self.person_repository.get_by_ids(&person_ids, languages).await?;
    let roles = self.role_repository.get_by_ids(&role_ids, languages).await?;
    let involved_map = involved_to_map(&involved, &people, &roles);
    let items: Vec<BookInvolved> = involved_map
      .get(&book_id)
      .expect("book id should be included in involved")
      .clone()
      .into_iter()
      .map(|(person, roles)| BookInvolved { person, roles })
      .collect();

    Ok(items)
  }

  async fn get_by_ids(
    &self,
    book_ids: &[u32],
    languages: &[Language],
  ) -> Result<HashMap<u32, Vec<BookInvolved>>, Box<dyn Error>> {
    let book_ids = to_i32(book_ids);

    let involved = Select::new::<DbBookInvolved>()
      .columns::<DbBookInvolved>(DbBookInvolved::TABLE_NAME)
      .where_expression(Expression::new(ValueIn::new(
        (DbBookInvolved::TABLE_NAME, "book_id"),
        &book_ids,
      )))
      .query_destruct(self.client)
      .await?;
    if involved.is_empty() {
      return Ok(book_ids.iter().map(|x| (*x as u32, Vec::new())).collect());
    }

    let mut person_ids: Vec<u32> = involved.iter().map(|x| x.person_id as u32).collect();
    person_ids.sort_unstable();
    person_ids.dedup();

    let mut role_ids: Vec<u32> = involved.iter().map(|x| x.role_id as u32).collect();
    role_ids.sort_unstable();
    role_ids.dedup();

    let people = self.person_repository.get_by_ids(&person_ids, languages).await?;
    let roles = self.role_repository.get_by_ids(&role_ids, languages).await?;

    let items: HashMap<u32, Vec<BookInvolved>> = involved_to_map(&involved, &people, &roles)
      .into_iter()
      .map(|(id, involved)| {
        (
          id as u32,
          involved
            .into_iter()
            .map(|(person, roles)| BookInvolved { person, roles })
            .collect(),
        )
      })
      .collect();
    Ok(items)
  }

  async fn filter_existing(&self, book_id: u32, involved: &[InvolvedId]) -> Result<Vec<InvolvedId>, Box<dyn Error>> {
    let book_id = book_id as i32;
    let involved: Vec<(i32, i32)> = involved
      .iter()
      .map(|x| (x.person_id as i32, x.role_id as i32))
      .collect();

    let filtered = Select::new::<DbBookInvolved>()
      .column::<i32>(DbBookInvolved::TABLE_NAME, "person_id")
      .column::<i32>(DbBookInvolved::TABLE_NAME, "role_id")
      .where_expression(involved_fks_in_ids(&involved))
      .where_expression(book_id_equal_id(book_id))
      .query(self.client)
      .await?
      .into_iter()
      .map(|(x, y)| InvolvedId {
        person_id: x as u32,
        role_id: y as u32,
      })
      .collect();
    Ok(filtered)
  }
}

fn involved_to_map(
  book_ids: &[DbBookInvolved],
  people: &[Person],
  roles: &[Role],
) -> HashMap<i32, Vec<(Person, Vec<Role>)>> {
  let mut result: HashMap<i32, HashMap<i32, Vec<i32>>> = HashMap::new();
  for book_involved in book_ids {
    involved_ids_to_map(&mut result, book_involved);
  }
  let mut result_result = HashMap::new();
  for (book_id, person_map) in result {
    let mut mapped_hashmap = Vec::with_capacity(person_map.len());
    for (person_id, roles_vec) in person_map {
      mapped_hashmap.push((
        people
          .iter()
          .find(|x| x.id as i32 == person_id)
          .expect("person should be included in people")
          .clone(),
        roles_vec
          .into_iter()
          .map(|x| {
            roles
              .iter()
              .find(|y| y.id as i32 == x)
              .expect("role should be included in roles")
              .clone()
          })
          .collect(),
      ));
    }
    result_result.insert(book_id, mapped_hashmap);
  }
  result_result
}
fn involved_ids_to_map(result: &mut HashMap<i32, HashMap<i32, Vec<i32>>>, book_involved: &DbBookInvolved) {
  let book_item = result.get_mut(&book_involved.book_id);
  let Some(person_map) = book_item else {
    let mut person_map = HashMap::new();
    person_map.insert(book_involved.person_id, vec![book_involved.role_id]);
    result.insert(book_involved.book_id, person_map);
    return;
  };

  let person_item = person_map.get_mut(&book_involved.person_id);
  let Some(roles) = person_item else {
    person_map.insert(book_involved.person_id, vec![book_involved.role_id]);
    return;
  };
  if roles.contains(&book_involved.role_id) {
    return;
  }
  roles.push(book_involved.role_id);
}
fn book_id_equal_id<'a>(book_id: i32) -> Expression<'a> {
  Expression::new(ValueEqual::new((DbBookInvolved::TABLE_NAME, "book_id"), book_id))
}
fn involved_fks_in_ids(involved_ids: &[(i32, i32)]) -> Expression<'_> {
  Expression::new(ValueIn::new(
    (
      (DbBookInvolved::TABLE_NAME, "person_id"),
      (DbBookInvolved::TABLE_NAME, "role_id"),
    ),
    involved_ids,
  ))
}
