use domain::entities::franchise::Franchise;
use domain::entities::image::image_data::ImageData;
use domain::entities::image::Image;
use domain::entities::book::Book;
use domain::entities::genre::Genre;
use domain::entities::character::Character;
use domain::entities::person::Person;
use domain::entities::theme::Theme;
use domain::items_total::BooksTotal;
use crate::controllers::book::doc::BookDoc;

#[derive(utoipa::OpenApi)]
#[openapi(info(title = "mycollection"),
nest(
("/books", BookDoc)
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
