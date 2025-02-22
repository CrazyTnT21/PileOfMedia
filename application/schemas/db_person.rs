use chrono::NaiveDate;
use domain::available_translations::AvailableTranslations;
use domain::entities::image::Image;
use domain::entities::person::Person;
use domain::entities::person::person_translation::PersonTranslation;
use from_row::FromRow;
use tokio_postgres::Row;

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
  pub fn to_entity(self, translations: AvailableTranslations<PersonTranslation>, image: Option<Image>) -> Person {
    Person {
      id: self.id as u32,
      name: self.name,
      first_name: self.first_name,
      last_name: self.last_name,
      birthday: self.birthday,
      height_cm: self.height.map(|x| x as u16),
      image,
      translations,
    }
  }
}
