use domain::entities::account::{Account, Email, Password};
use domain::entities::user::User;
use from_row::FromRow;
use tokio_postgres::Row;

#[derive(FromRow, Debug)]
#[rename = "account"]
pub struct DbAccount {
  pub user_id: i32,
  pub email: String,
  pub password: String,
}

impl DbAccount {
  pub fn to_entity(self, user: User) -> Account {
    Account {
      user,
      email: Email(self.email),
      password: Password(self.password),
    }
  }
}
