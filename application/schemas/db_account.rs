use domain::entities::account::{Account, Password};
use domain::entities::user::User;
use from_row::FromRow;
use tokio_postgres::Row;

#[derive(FromRow, Debug)]
#[rename = "account"]
pub struct DbAccount {
  pub user_id: i32,
  pub password: String,
}

impl DbAccount {
  pub fn to_entity(self, user: User) -> Account {
    Account {
      user,
      password: Password(self.password),
    }
  }
}
