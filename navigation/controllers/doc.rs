use crate::controllers::account_controller::LoginReturnData;
use domain::entities::book::book_involved::InvolvedId;
use domain::entities::account::Email;
use domain::entities::image::create_image::Base64Data;
use domain::entities::user::create_user::CreateUser;
use domain::entities::account::create_account::CreateAccount;
use crate::controllers::account_controller::account_doc::AccountDoc;
use crate::controllers::account_controller::LoginData;
use domain::entities::user::User;
use crate::controllers::user_controller::user_doc::UserDoc;
use domain::items_total::UsersTotal;
use domain::entities::role::Role;
use crate::controllers::role_controller::role_doc::RoleDoc;
use domain::entities::book::book_involved::BookInvolved;
use domain::entities::franchise::Franchise;
use domain::entities::image::image_data::ImageData;
use domain::entities::image::Image;
use domain::entities::book::Book;
use domain::entities::genre::Genre;
use domain::entities::character::Character;
use domain::entities::person::Person;
use domain::entities::theme::Theme;
use domain::items_total::BooksTotal;
use domain::items_total::GenresTotal;
use domain::items_total::ThemesTotal;
use domain::items_total::PeopleTotal;
use crate::controllers::book_controller::book_doc::BookDoc;
use crate::controllers::genre_controller::genre_doc::GenreDoc;
use crate::controllers::theme_controller::theme_doc::ThemeDoc;
use crate::controllers::person_controller::person_doc::PersonDoc;
use crate::controllers::character_controller::character_doc::CharacterDoc;
use domain::items_total::CharactersTotal;
use domain::items_total::RolesTotal;
use domain::items_total::BookInvolvedTotal;
use domain::entities::book::book_character::BookCharacter;
use domain::items_total::BookCharactersTotal;
use domain::entities::person::person_role::PersonRole;
use domain::entities::image::create_image::CreateImage;
use domain::entities::account::Password;
use crate::controllers::franchise_controller::franchise_doc::FranchiseDoc;

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
    UsersTotal
  )))]
pub(crate) struct ApiDoc;
