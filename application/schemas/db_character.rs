use chrono::NaiveDate;
use domain::available_translations::AvailableTranslations;
use domain::entities::character::character_translation::CharacterTranslation;
use domain::entities::character::Character;
use domain::entities::image::Image;
use from_row::FromRow;
use tokio_postgres::Row;

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
  pub fn to_entity(self, translations: AvailableTranslations<CharacterTranslation>, image: Option<Image>) -> Character {
    Character {
      id: self.id as u32,
      birthday: self.birthday,
      height_cm: self.height.map(|x| x as u32),
      image,
      translations,
    }
  }
}
