use chrono::NaiveDate;
use domain::entities::image::Image;
use tokio_postgres::Row;

use domain::entities::user::User;
use from_row::FromRow;

#[derive(FromRow, Debug)]
#[rename = "\"user\""]
pub struct DbUser {
  pub id: i32,
  pub name: String,
  pub joined: NaiveDate,
  pub description: Option<String>,
  pub profile_picture_id: Option<i32>,
}

impl DbUser {
  pub fn to_entity(self, profile_picture: Option<Image>) -> User {
    User {
      id: self.id as u32,
      name: self.name,
      joined: self.joined,
      description: self.description,
      profile_picture,
    }
  }
}
