use chrono::NaiveDate;

use crate::entities::franchise::franchise::Franchise;
use crate::entities::image::image::Image;

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct Book {
  pub id: i32,
  pub title: String,
  pub description: Option<String>,
  pub chapters: Option<i16>,
  pub pages: Option<i16>,
  pub words: Option<i32>,
  #[cfg_attr(feature = "utoipa", schema(schema_with = crate::utoipa::utoipa::naive_date_optional))]
  pub published: Option<NaiveDate>,
  pub cover: Image,
  pub score: f32,
  pub rank: i32,
  pub popularity: i32,
  pub favorites: i32,
  pub members: i32,
  #[cfg_attr(feature = "utoipa", schema(schema_with = crate::utoipa::utoipa::naive_date))]
  pub added: NaiveDate,
  pub franchise: Option<Franchise>,
}
