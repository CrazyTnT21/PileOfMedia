use chrono::NaiveDate;
use tokio_postgres::Row;

use domain::entities::image::Image;
use domain::entities::person::{Person};
use from_row::FromRow;

use crate::schemas::db_person_translation::DbPersonTranslation;

#[derive(FromRow, Debug)]
#[rename = "person"]
pub struct DbPerson {
  pub id: i32,
  pub name: String,
  #[rename = "firstname"]
  pub first_name: Option<String>,
  #[rename = "lastname"]
  pub last_name: Option<String>,
  pub birthday: Option<NaiveDate>,
  pub height: Option<i16>,
  #[rename = "fkimage"]
  pub fk_image: Option<i32>,
}

impl DbPerson {
  pub fn to_entity(self, translation: DbPersonTranslation, image: Option<Image>) -> Person {
    Person {
      id: self.id as u32,
      name: self.name,
      first_name: self.first_name,
      last_name: self.last_name,
      description: translation.description,
      birthday: self.birthday,
      height: self.height.map(|x| x as u16),
      image,
    }
  }
}
