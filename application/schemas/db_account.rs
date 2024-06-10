use tokio_postgres::Row;
use domain::entities::account::{Account, Email, Password};
use domain::entities::user::User;
use from_row::FromRow;

#[derive(FromRow, Debug)]
#[rename = "account"]
pub struct DbAccount {
  #[rename = "fkuser"]
  pub fk_user: i32,
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
