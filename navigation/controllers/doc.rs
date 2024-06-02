use domain::entities::franchise::Franchise;
use domain::entities::image::image_data::ImageData;
use domain::entities::image::Image;
use domain::entities::book::Book;
use domain::entities::genre::Genre;
use domain::entities::character::Character;
use domain::entities::person::Person;
use domain::entities::theme::Theme;
use domain::items_total::BooksTotal;
use crate::controllers::book_controller::book_doc::BookDoc;
use crate::controllers::genre_controller::genre_doc::GenreDoc;
use crate::controllers::theme_controller::theme_doc::ThemeDoc;

#[derive(utoipa::OpenApi)]
#[openapi(info(title = "mycollection"),
nest(
("/books", BookDoc),
("/genres", GenreDoc),
("/themes", ThemeDoc),
),
components(schemas(
Genre,
Character,
Person,
Theme,
Book,
Image,
ImageData,
Franchise,
BooksTotal
)))]
pub(crate) struct ApiDoc;
