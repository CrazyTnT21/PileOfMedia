use std::error::Error;
use std::sync::Arc;

use async_trait::async_trait;
use tokio_postgres::Client;

use domain::entities::book::book_involved::{BookInvolved, InvolvedId};
use domain::entities::person::person_role::PersonRole;
use domain::enums::language::Language;
use domain::items_total::ItemsTotal;
use domain::pagination::Pagination;
use from_row::{FromRow, Table};
use repositories::book_repository::book_involved_repository::BookInvolvedRepository;
use repositories::book_repository::BookRepository;
use repositories::person_repository::PersonRepository;
use repositories::role_repository::RoleRepository;

use crate::enums::db_language::DbLanguage;
use crate::schemas::db_book_involved::DbBookInvolved;
use crate::schemas::db_role::DbRole;
use crate::schemas::db_role_translation::DbRoleTranslation;
use crate::select::combined_tuple::CombinedType;
use crate::select::conditions::column_equal::ColumnEqual;
use crate::select::conditions::column_null::ColumnNull;
use crate::select::conditions::value_equal::ValueEqual;
use crate::select::conditions::value_in::ValueIn;
use crate::select::expression::Expression;
use crate::select::Select;

pub struct DefaultBookInvolvedRepository<'a> {
  client: &'a Client,
  default_language: DbLanguage,
  book_repository: Arc<dyn BookRepository + 'a>,
  person_repository: Arc<dyn PersonRepository + 'a>,
  role_repository: Arc<dyn RoleRepository + 'a>,
}

impl<'a> DefaultBookInvolvedRepository<'a> {
  pub fn new(
    client: &'a Client,
    default_language: Language,
    book_repository: Arc<dyn BookRepository + 'a>,
    person_repository: Arc<dyn PersonRepository + 'a>,
    role_repository: Arc<dyn RoleRepository + 'a>,
  ) -> DefaultBookInvolvedRepository<'a> {
    DefaultBookInvolvedRepository {
      client,
      default_language: default_language.into(),
      book_repository,
      person_repository,
      role_repository,
    }
  }
}

#[async_trait]
impl BookInvolvedRepository for DefaultBookInvolvedRepository<'_> {
  async fn get(
    &self,
    book_id: u32,
    language: Language,
    pagination: Pagination,
  ) -> Result<ItemsTotal<BookInvolved>, Box<dyn Error>> {
    let book_id = book_id as i32;
    let db_language = DbLanguage::from(language);
    let total = Select::new::<DbBookInvolved>()
      .count()
      .transform(|x| involved_joins(x, &db_language, &self.default_language))
      .where_expression(Expression::new(ValueEqual::new(
        (DbBookInvolved::TABLE_NAME, "fkbook"),
        book_id,
      )))
      .get_single(self.client)
      .await?
      .expect("Count should return one row");
    let total = total.0 as usize;

    let involved = Select::new::<DbBookInvolved>()
      .columns::<DbRole>(DbRole::TABLE_NAME)
      .columns::<Option<DbRoleTranslation>>("role_translation")
      .columns::<Option<DbRoleTranslation>>("role_translation_fallback")
      .column::<i32>(DbBookInvolved::TABLE_NAME, "fkperson")
      .column::<i32>(DbBookInvolved::TABLE_NAME, "fkrole")
      .transform(|x| involved_joins(x, &db_language, &self.default_language))
      .where_expression(Expression::new(ValueEqual::new(
        (DbBookInvolved::TABLE_NAME, "fkbook"),
        book_id,
      )))
      .pagination(pagination)
      .query(self.client)
      .await?;

    if involved.is_empty() {
      return Ok(ItemsTotal { items: vec![], total });
    }
    let mut person_ids: Vec<u32> = involved.iter().map(|x| x.3 as u32).collect();
    person_ids.sort_unstable();
    person_ids.dedup();

    let mut role_ids: Vec<u32> = involved.iter().map(|x| x.4 as u32).collect();
    role_ids.sort_unstable();
    role_ids.dedup();

    let people = self.person_repository.get_by_ids(&person_ids, language).await?;
    let roles = self.role_repository.get_by_ids(&role_ids, language).await?;

    let items: Vec<BookInvolved> = involved
      .iter()
      .map(|x| {
        let person = people.iter().find(|y| y.id == x.3 as u32).unwrap().clone();
        let role = roles.iter().find(|y| y.id == x.4 as u32).unwrap().clone();

        BookInvolved {
          person,
          role: PersonRole { role },
        }
      })
      .collect();

    Ok(ItemsTotal { items, total })
  }

  async fn filter_existing(&self, book_id: u32, involved: &[InvolvedId]) -> Result<Vec<InvolvedId>, Box<dyn Error>> {
    let book_id = book_id as i32;
    let involved: Vec<(i32, i32)> = involved
      .iter()
      .map(|x| (x.person_id as i32, x.role_id as i32))
      .collect();

    let filtered = Select::new::<DbBookInvolved>()
      .column::<i32>(DbBookInvolved::TABLE_NAME, "fkperson")
      .column::<i32>(DbBookInvolved::TABLE_NAME, "fkrole")
      .where_expression(Expression::new(ValueIn::new(
        (
          (DbBookInvolved::TABLE_NAME, "fkperson"),
          (DbBookInvolved::TABLE_NAME, "fkrole"),
        ),
        &involved,
      )))
      .where_expression(Expression::column_equal(DbBookInvolved::TABLE_NAME, "fkbook", book_id))
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

fn involved_joins<'a, T: FromRow<DbType = T> + CombinedType>(
  select: Select<'a, T>,
  language: &'a DbLanguage,
  fallback_language: &'a DbLanguage,
) -> Select<'a, T> {
  select
    .inner_join::<DbRole>(
      None,
      Expression::new(ColumnEqual::new(
        (DbRole::TABLE_NAME, "id"),
        (DbBookInvolved::TABLE_NAME, "fkrole"),
      )),
    )
    .left_join::<DbRoleTranslation>(
      Some("role_translation"),
      Expression::new(ColumnEqual::new(
        ("role_translation", "fktranslation"),
        (DbRole::TABLE_NAME, "id"),
      ))
      .and(Expression::column_equal("role_translation", "language", language)),
    )
    .left_join::<DbRoleTranslation>(
      Some("role_translation_fallback"),
      Expression::new(ColumnEqual::new(
        ("role_translation_fallback", "fktranslation"),
        (DbRole::TABLE_NAME, "id"),
      ))
      .and(Expression::column_equal(
        "role_translation_fallback",
        "language",
        fallback_language,
      ))
      .and(Expression::new(ColumnNull::new(("role_translation", "fktranslation")))),
    )
}
