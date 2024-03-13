use std::sync::Arc;

use actix_web::{App, HttpServer, web};
use actix_web::web::Data;

use crate::application::default_book_repository::DefaultBookRepository;
use crate::infrastructure::default_book_service::DefaultBookService;
use crate::navigation::controllers;
use crate::traits::book_repository::BookRepository;
use crate::traits::book_service::BookService;

mod navigation;
mod domain;
mod traits;
mod application;
mod infrastructure;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  HttpServer::new(move || {
    App::new()
      .configure(controllers::add_controllers)
      .configure(register_dependencies)
  })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

fn register_dependencies(config: &mut web::ServiceConfig) {
  register_book_service(config);
}

fn register_book_service(config: &mut web::ServiceConfig) {
  let book_service_arc: Arc<dyn BookService> = Arc::new(DefaultBookService::new(get_book_repository()));
  let data = Data::from(book_service_arc);
  config.app_data(Data::clone(&data));
}

fn get_book_repository() -> Box<dyn BookRepository> {
  Box::new(DefaultBookRepository)
}
