use domain::entities::genre::create_genre::CreateGenre;
use domain::entities::person::create_person::CreatePerson;
use domain::entities::account::create_account::CreateAccount;
use domain::entities::account::Email;
use domain::entities::account::Password;
use domain::entities::book::Book;
use domain::entities::book::book_character::BookCharacter;
use domain::entities::book::book_involved::BookInvolved;
use domain::entities::book::book_involved::InvolvedId;
use domain::entities::book::create_book::CreateBook;
use domain::entities::book::create_book::CreateBookTranslation;
use domain::entities::book::create_book::CreateCover;
use domain::entities::character::Character;
use domain::entities::franchise::Franchise;
use domain::entities::person::create_person::CreatePersonTranslation;
use domain::entities::genre::Genre;
use domain::entities::image::create_image::Base64Data;
use domain::entities::image::create_image::CreateImage;
use domain::entities::image::Image;
use domain::entities::image::image_data::ImageData;
use domain::entities::person::Person;
use domain::entities::person::person_role::PersonRole;
use domain::entities::role::Role;
use domain::entities::theme::Theme;
use domain::entities::user::create_user::CreateUser;
use domain::entities::user::User;
use domain::enums::language::Language;
use domain::items_total::BookCharactersTotal;
use domain::items_total::BookInvolvedTotal;
use domain::items_total::BooksTotal;
use domain::items_total::CharactersTotal;
use domain::items_total::GenresTotal;
use domain::items_total::PeopleTotal;
use domain::items_total::RolesTotal;
use domain::items_total::ThemesTotal;
use domain::items_total::UsersTotal;

use domain::entities::genre::create_genre::CreateGenreTranslation;
use crate::controllers::account_controller::account_doc::AccountDoc;
use crate::controllers::account_controller::LoginData;
use crate::controllers::account_controller::LoginReturnData;
use crate::controllers::book_controller::book_doc::BookDoc;
use crate::controllers::character_controller::character_doc::CharacterDoc;
use crate::controllers::franchise_controller::franchise_doc::FranchiseDoc;
use crate::controllers::genre_controller::genre_doc::GenreDoc;
use crate::controllers::person_controller::person_doc::PersonDoc;
use crate::controllers::role_controller::role_doc::RoleDoc;
use crate::controllers::theme_controller::theme_doc::ThemeDoc;
use crate::controllers::user_controller::user_doc::UserDoc;

#[derive(utoipa::OpenApi)]
#[openapi(info(title = "mycollection"),
  nest(
    ("/books", BookDoc),
    ("/genres", GenreDoc),
    ("/themes", ThemeDoc),
    ("/people", PersonDoc),
    ("/characters", CharacterDoc),
    ("/roles", RoleDoc),
    ("/users", UserDoc),
    ("/franchises", FranchiseDoc),
    ("/accounts", AccountDoc),
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
    Role,
    PersonRole,
    BookInvolved,
    BookCharacter,
    User,
    InvolvedId,
    CreateAccount,
    CreateUser,
    CreateImage,
    Base64Data,
    Email,
    Password,
    LoginData,
    LoginReturnData,
    BooksTotal,
    GenresTotal,
    ThemesTotal,
    PeopleTotal,
    CharactersTotal,
    RolesTotal,
    BookInvolvedTotal,
    BookCharactersTotal,
    UsersTotal,
    Language,
    CreateBook,
    CreateBookTranslation,
    CreateCover,
    CreatePerson,
    CreatePersonTranslation,
    CreateGenre,
    CreateGenreTranslation
  )))]
pub(crate) struct ApiDoc;
