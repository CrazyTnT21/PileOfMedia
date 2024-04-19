use serde::Serialize;
use chrono::NaiveDate;
use crate::entities::franchise::franchise::Franchise;
use crate::entities::image::image::Image;

#[derive(Serialize)]
pub struct Book {
  pub id: i32,
  pub title: String,
  pub description: Option<String>,
  pub chapters: Option<i16>,
  pub pages: Option<i16>,
  pub words: Option<i32>,
  pub published: Option<NaiveDate>,
  pub cover: Image,
  pub score: f32,
  pub rank: i32,
  pub popularity: i32,
  pub favorites: i32,
  pub members: i32,
  pub added: NaiveDate,
  pub franchise: Option<Franchise>,
}
