use chrono::NaiveDate;
use tokio_postgres::Row;

use domain::entities::character::Character;
use domain::entities::image::Image;
use from_row::FromRow;

use crate::schemas::db_character_translation::DbCharacterTranslation;

#[derive(FromRow, Debug)]
#[rename = "character"]
pub struct DbCharacter {
  pub id: i32,
  pub birthday: Option<NaiveDate>,
  pub height: Option<i32>,
  #[rename = "fkimage"]
  pub fk_image: Option<i32>,
}

impl DbCharacter {
  pub fn to_entity(self, translation: DbCharacterTranslation, image: Option<Image>) -> Character {
    Character {
      id: self.id as u32,
      name: translation.name,
      first_name: translation.first_name,
      last_name: translation.last_name,
      description: translation.description,
      birthday: self.birthday,
      height_cm: self.height.map(|x| x as u32),
      image,
      language: translation.language.into(),
    }
  }
}
