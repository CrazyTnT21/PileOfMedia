use std::error::Error;

use actix_web::{get, Responder, web};
use actix_web::http::header::AcceptLanguage;
use web::{Data, Header, Path};

use crate::navigation::controllers::{DEFAULT_LANGUAGE, format_content_language, get_language_and_fallback};
use crate::traits::book_service::BookService;

pub fn add_routes(config: &mut web::ServiceConfig) {
  config.service(
    web::scope("/books").service((get_by_id, get_by_title, get))
  );
}

#[get("")]
async fn get(mut accept_language: Header<AcceptLanguage>, book_service: Data<dyn BookService>) -> Result<impl Responder, Box<dyn Error>> {
  let (language, fallback_language) = get_language_and_fallback(&mut accept_language, DEFAULT_LANGUAGE);
  println!("Route for books in {} and fallback {:?}", language, fallback_language);
  Ok(web::Json(book_service.get(language, fallback_language, 0, 50)?)
    .customize()
    .insert_header(("content-language", format_content_language(language, fallback_language))))
}

#[get("{id:\\d+}")]
async fn get_by_id(path: Path<u32>, mut accept_language: Header<AcceptLanguage>, book_service: Data<dyn BookService>) -> Result<impl Responder, Box<dyn Error>> {
  let (language, fallback_language) = get_language_and_fallback(&mut accept_language, DEFAULT_LANGUAGE);
  let id = path.into_inner();
  println!("Route for a book with id {} in {} and fallback {:?}", id, language, fallback_language);

  Ok(web::Json(book_service.get_by_id(id, language, fallback_language)?)
    .customize()
    .insert_header(("content-language", format_content_language(language, fallback_language))))
}

#[get("{title}")]
async fn get_by_title(path: Path<String>, mut accept_language: Header<AcceptLanguage>, book_service: Data<dyn BookService>) -> Result<impl Responder, Box<dyn Error>> {
  let title = path.into_inner();

  let (language, fallback_language) = get_language_and_fallback(&mut accept_language, DEFAULT_LANGUAGE);
  println!("Route for books with the title {} in {} and fallback {:?}", title, language, fallback_language);

  Ok(web::Json(book_service.get_by_title(&title, language, fallback_language, 0, 50)?)
    .customize()
    .insert_header(("content-language", format_content_language(language, fallback_language))))
}

