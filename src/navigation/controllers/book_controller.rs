use std::fmt::Display;
use std::io::Error;

use actix_web::{get, Responder, web};
use chrono::NaiveDate;

use crate::domain::entities::book::book::Book;
use crate::domain::entities::franchise::franchise::Franchise;
use crate::domain::entities::image::image::{Image, ImageExtension};
use crate::domain::enums::language::Language;

pub fn add_routes(config: &mut web::ServiceConfig) {
  config.service(
    web::scope("/books").service((get_by_id, get_by_title, get))
  );
}

#[get("{language}")]
async fn get(path: web::Path<Language>) -> Result<impl Responder, Error> {
  let language = path.into_inner();
  println!("Route for books in {}", language);
  Ok(web::Json(vec![fake_book()]))
}

#[get("{language}/{id:\\d+}")]
async fn get_by_id(path: web::Path<(Language, u32)>) -> Result<impl Responder, Error> {
  let (language, id) = path.into_inner();
  println!("Route for a book with id {} in {}", id, language);
  Ok(web::Json(fake_book()))
}

#[get("{language}/{title}")]
async fn get_by_title(path: web::Path<(Language, String)>) -> Result<impl Responder, Error> {
  let (language, title) = path.into_inner();
  println!("Route for books with the title {} in {}", title, language);
  Ok(web::Json(vec![fake_book(), fake_book(), fake_book()]))
}

fn fake_book() -> Book {
  Book {
    cover: Image {
      extension: ImageExtension::JPG,
      height: 100,
      id: 1,
      uri: String::from("https://Test"),
      width: 100,
    },
    id: 1,
    description: Some(String::from("Book description")),
    pages: Some(10),
    title: String::from("Book title"),
    words: Some(10),
    chapters: Some(1),
    published: NaiveDate::from_ymd_opt(2024, 2, 17),
    added: NaiveDate::MAX,
    members: 105,
    favorites: 2,
    rank: 6323,
    score: 6.25,
    popularity: 5545,
    franchise: Some(Franchise {
      id: 1,
      name: String::from("Book franchise"),
    }),
  }
}
