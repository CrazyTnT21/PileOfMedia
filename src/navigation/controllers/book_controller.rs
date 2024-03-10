use std::error::Error;

use actix_web::{get, Responder, web};
use actix_web::http::header::AcceptLanguage;
use chrono::NaiveDate;

use crate::domain::entities::book::book::Book;
use crate::domain::entities::franchise::franchise::Franchise;
use crate::domain::entities::image::image::{Image, ImageExtension};
use crate::navigation::controllers::{DEFAULT_LANGUAGE, format_content_language, get_language_and_fallback};

pub fn add_routes(config: &mut web::ServiceConfig) {
  config.service(
    web::scope("/books").service((get_by_id, get_by_title, get))
  );
}

#[get("")]
async fn get(mut accept_language: web::Header<AcceptLanguage>) -> Result<impl Responder, Box<dyn Error>> {
  let (language, fallback_language) = get_language_and_fallback(&mut accept_language, DEFAULT_LANGUAGE);
  println!("Route for books in {} and fallback {:?}", language, fallback_language);

  Ok(web::Json(vec![fake_book()])
    .customize()
    .insert_header(("content-language", format_content_language(language, fallback_language))))
}

#[get("{id:\\d+}")]
async fn get_by_id(path: web::Path<u32>, mut accept_language: web::Header<AcceptLanguage>) -> Result<impl Responder, Box<dyn Error>> {
  let (language, fallback_language) = get_language_and_fallback(&mut accept_language, DEFAULT_LANGUAGE);
  let id = path.into_inner();
  println!("Route for a book with id {} in {} and fallback {:?}", id, language, fallback_language);
  Ok(web::Json(fake_book()))
}

#[get("{title}")]
async fn get_by_title(path: web::Path<String>, mut accept_language: web::Header<AcceptLanguage>) -> Result<impl Responder, Box<dyn Error>> {
  let title = path.into_inner();

  let (language, fallback_language) = get_language_and_fallback(&mut accept_language, DEFAULT_LANGUAGE);
  println!("Route for books with the title {} in {} and fallback {:?}", title, language, fallback_language);
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
