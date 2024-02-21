use actix_web::web;

mod book_controller;

pub fn add_controllers(config: &mut web::ServiceConfig) {
  config.configure(book_controller::add_routes);
}
