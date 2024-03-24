use std::sync::Arc;
use axum::Router;

use crate::application::default_book_repository::DefaultBookRepository;
use crate::infrastructure::default_book_service::DefaultBookService;
use crate::navigation::controllers::add_controllers;
use crate::traits::book_repository::BookRepository;
use crate::traits::book_service::BookService;

mod navigation;
mod domain;
mod traits;
mod application;
mod infrastructure;

#[tokio::main]
async fn main() -> std::io::Result<()> {
  let app = add_controllers(Router::new());

  let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
  axum::serve(listener, app).await
}

fn get_book_service() -> Arc<dyn BookService> {
  Arc::new(DefaultBookService::new(get_book_repository()))
}

fn get_book_repository() -> Arc<dyn BookRepository> {
  Arc::new(DefaultBookRepository)
}
