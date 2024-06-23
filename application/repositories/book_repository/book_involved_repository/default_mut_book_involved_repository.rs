use std::error::Error;

use async_trait::async_trait;
use tokio_postgres::Transaction;
use domain::entities::book::book_involved::InvolvedId;

use from_row::Table;
use repositories::book_repository::book_involved_repository::mut_book_involved_repository::MutBookInvolvedRepository;

use crate::delete::Delete;
use crate::insert::Insert;
use crate::schemas::db_book_involved::DbBookInvolved;
use crate::select::conditions::value_in::ValueIn;
use crate::select::expression::Expression;

pub struct DefaultMutBookInvolvedRepository<'a> {
  transaction: &'a Transaction<'a>,
}

impl<'a> DefaultMutBookInvolvedRepository<'a> {
  pub fn new(transaction: &'a Transaction<'a>) -> DefaultMutBookInvolvedRepository<'a> {
    DefaultMutBookInvolvedRepository { transaction }
  }
}

#[async_trait]
impl<'a> MutBookInvolvedRepository for DefaultMutBookInvolvedRepository<'a> {
  async fn add(&self, book_id: u32, involved: &[InvolvedId]) -> Result<(), Box<dyn Error>> {
    let book_id = book_id as i32;
    let involved: Vec<(i32, i32)> = involved.iter()
      .map(|x| (x.person_id as i32, x.role_id as i32))
      .collect();
    let mut insert = Insert::new::<DbBookInvolved>(["fkbook", "fkperson", "fkrole"]);
    involved.iter().for_each(|(x, y)| { insert.push_as_ref([&book_id, x, y]); });
    insert.execute_transaction(self.transaction).await?;
    Ok(())
  }

  async fn remove(&self, book_id: u32, involved: &[InvolvedId]) -> Result<(), Box<dyn Error>> {
    let book_id = book_id as i32;
    let involved: Vec<(i32, i32)> = involved.iter().map(|x| (x.person_id as i32, x.role_id as i32)).collect();

    Delete::new::<DbBookInvolved>(
      Expression::column_equal(DbBookInvolved::TABLE_NAME, "fkbook", &book_id)
        .and(Expression::new(ValueIn::new(((DbBookInvolved::TABLE_NAME, "fkperson"), (DbBookInvolved::TABLE_NAME, "fkrole")), &involved)))
    )
      .execute_transaction(self.transaction)
      .await?;
    Ok(())
  }
}
