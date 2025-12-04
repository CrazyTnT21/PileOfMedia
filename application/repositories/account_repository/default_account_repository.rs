use std::error::Error;
use std::sync::Arc;

use async_trait::async_trait;
use tokio_postgres::Client;

use domain::entities::account::{Account};
use domain::entities::user::User;
use domain::items_total::ItemsTotal;
use domain::pagination::Pagination;
use from_row::Table;
use repositories::account_repository::AccountRepository;
use repositories::user_repository::UserRepository;

use crate::convert_to_sql::to_i32;
use crate::schemas::db_account::DbAccount;
use crate::schemas::db_user::DbUser;
use crate::select::Select;
use crate::select::conditions::value_equal::ValueEqual;
use crate::select::conditions::value_in::ValueIn;
use crate::select::expression::Expression;

pub struct DefaultAccountRepository<'a> {
  client: &'a Client,
  user_repository: Arc<dyn UserRepository + 'a>,
}

impl<'a> DefaultAccountRepository<'a> {
  pub fn new(client: &'a Client, user_repository: Arc<dyn UserRepository + 'a>) -> DefaultAccountRepository<'a> {
    DefaultAccountRepository {
      client,
      user_repository,
    }
  }
}

#[async_trait]
impl AccountRepository for DefaultAccountRepository<'_> {
  async fn get(&self, pagination: Pagination) -> Result<ItemsTotal<Account>, Box<dyn Error>> {
    let total = Select::new::<DbAccount>().query_count(self.client).await?;

    let accounts = Select::new::<DbAccount>()
      .columns::<DbAccount>(DbAccount::TABLE_NAME)
      .pagination(pagination)
      .query(self.client)
      .await?;

    Ok(ItemsTotal {
      items: self.to_entities(accounts).await?,
      total,
    })
  }

  async fn get_by_user_id(&self, id: u32) -> Result<Option<Account>, Box<dyn Error>> {
    let id = id as i32;
    let account = Select::new::<DbAccount>()
      .columns::<DbAccount>(DbAccount::TABLE_NAME)
      .where_expression(Expression::new(ValueEqual::new((DbAccount::TABLE_NAME, "user_id"), id)))
      .get_single(self.client)
      .await?;

    match account {
      None => Ok(None),
      Some(value) => {
        let user_id = value.0.user_id;
        let user = self
          .user_repository
          .get_by_id(user_id as u32)
          .await?
          .expect("Associated user has to exist");
        Ok(Some(to_entity(value, user)))
      }
    }
  }

  async fn get_by_user_ids(&self, ids: &[u32]) -> Result<Vec<Account>, Box<dyn Error>> {
    let ids = to_i32(ids);

    let accounts = Select::new::<DbAccount>()
      .columns::<DbAccount>(DbAccount::TABLE_NAME)
      .where_expression(Expression::new(ValueIn::new((DbAccount::TABLE_NAME, "user_id"), &ids)))
      .query(self.client)
      .await?;

    Ok(self.to_entities(accounts).await?)
  }

  async fn get_by_username(&self, name: &str) -> Result<Option<Account>, Box<dyn Error>> {
    let account = Select::new::<DbAccount>()
      .columns::<DbAccount>(DbAccount::TABLE_NAME)
      .inner_join::<DbUser>(None, Expression::value_equal(DbUser::TABLE_NAME, "name", name))
      .get_single(self.client)
      .await?;

    match account {
      None => Ok(None),
      Some(value) => {
        let user_id = value.0.user_id;
        let user = self
          .user_repository
          .get_by_id(user_id as u32)
          .await?
          .expect("Associated user has to exist");
        Ok(Some(to_entity(value, user)))
      }
    }
  }

  async fn filter_existing(&self, users: &[u32]) -> Result<Vec<u32>, Box<dyn Error>> {
    let users = to_i32(users);

    let count = Select::new::<DbAccount>()
      .column::<i32>(DbAccount::TABLE_NAME, "user_id")
      .where_expression(Expression::new(ValueIn::new(
        (DbAccount::TABLE_NAME, "user_id"),
        &users,
      )))
      .query(self.client)
      .await?
      .into_iter()
      .map(|x| x.0 as u32)
      .collect();
    Ok(count)
  }
}

fn to_entity(account: (DbAccount,), user: User) -> Account {
  account.0.to_entity(user)
}

impl DefaultAccountRepository<'_> {
  async fn to_entities(&self, items: Vec<(DbAccount,)>) -> Result<Vec<Account>, Box<dyn Error>> {
    let user_ids: Vec<u32> = items.iter().map(|x| x.0.user_id as u32).collect();

    let mut users = match user_ids.is_empty() {
      true => vec![],
      false => self.user_repository.get_by_ids(&user_ids).await?,
    };
    Ok(
      items
        .into_iter()
        .map(|x| {
          let user_index = users
            .iter()
            .position(|y| y.id == x.0.user_id as u32)
            .expect("Associated user should exist");

          let user = users.swap_remove(user_index);
          x.0.to_entity(user)
        })
        .collect(),
    )
  }
}
