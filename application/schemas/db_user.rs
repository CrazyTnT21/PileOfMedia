use chrono::NaiveDate;
use tokio_postgres::Row;
use domain::entities::image::Image;

use domain::entities::user::User;
use from_row::FromRow;

#[derive(FromRow, Debug)]
#[rename = "\"User\""]
pub struct DbUser {
  pub id: i32,
  pub name: String,
  pub joined: NaiveDate,
  pub description: Option<String>,
  pub deleted: bool,
  #[rename = "fkprofilepicture"]
  pub fk_profile_picture: Option<i32>,
}

impl DbUser {
  pub fn to_entity(self, profile_picture: Option<Image>) -> User {
    User {
      id: self.id as u32,
      name: self.name,
      joined: self.joined,
      description: self.description,
      deleted: self.deleted,
      profile_picture,
    }
  }
}
