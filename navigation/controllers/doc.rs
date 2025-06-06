use domain::available_translations::BookAvailableTranslations;
use domain::available_translations::CharacterAvailableTranslations;
use domain::available_translations::FranchiseAvailableTranslations;
use domain::available_translations::GenreAvailableTranslations;
use domain::available_translations::PersonAvailableTranslations;
use domain::available_translations::RoleAvailableTranslations;
use domain::available_translations::ThemeAvailableTranslations;
use domain::entities::account::Email;
use domain::entities::account::Password;
use domain::entities::account::create_account::CreateAccount;
use domain::entities::account::create_account::CreateAccountData;
use domain::entities::book::Book;
use domain::entities::book::book_character::BookCharacter;
use domain::entities::book::book_statistic::BookStatistic;
use domain::entities::book::book_translation::BookTranslation;
use domain::entities::book::create_book::CreateBook;
use domain::entities::book::create_book::CreateBookData;
use domain::entities::book::create_book::CreateBookTranslation;
use domain::entities::book::create_book::CreateCover;
use domain::entities::character::Character;
use domain::entities::character::character_translation::CharacterTranslation;
use domain::entities::character::create_character::CreateCharacter;
use domain::entities::character::create_character::CreateCharacterData;
use domain::entities::character::create_character::CreateCharacterTranslation;
use domain::entities::franchise::Franchise;
use domain::entities::franchise::create_franchise::CreateFranchise;
use domain::entities::franchise::create_franchise::CreateFranchiseTranslation;
use domain::entities::franchise::franchise_translation::FranchiseTranslation;
use domain::entities::genre::Genre;
use domain::entities::genre::create_genre::CreateGenre;
use domain::entities::genre::create_genre::CreateGenreTranslation;
use domain::entities::genre::genre_translation::GenreTranslation;
use domain::entities::image::Image;
use domain::entities::image::create_image::CreateImage;
use domain::entities::image::image_data::ImageData;
use domain::entities::involved::Involved;
use domain::entities::involved::InvolvedId;
use domain::entities::person::Person;
use domain::entities::person::create_person::CreatePerson;
use domain::entities::person::create_person::CreatePersonData;
use domain::entities::person::create_person::CreatePersonTranslation;
use domain::entities::person::person_translation::PersonTranslation;
use domain::entities::rating::Rating;
use domain::entities::role::Role;
use domain::entities::role::create_role::CreateRole;
use domain::entities::role::create_role::CreateRoleTranslation;
use domain::entities::role::role_translation::RoleTranslation;
use domain::entities::theme::Theme;
use domain::entities::theme::create_theme::CreateTheme;
use domain::entities::theme::create_theme::CreateThemeTranslation;
use domain::entities::theme::theme_translation::ThemeTranslation;
use domain::entities::user::User;
use domain::entities::user::create_user::CreateUser;
use domain::entities::user::create_user::CreateUserData;
use domain::entities::user::create_user_book::CreateUserBook;
use domain::entities::user::user_book::UserBook;
use domain::entities::user::user_status::UserStatus;
use domain::enums::language::Language;
use domain::items_total::BooksTotal;
use domain::items_total::CharactersTotal;
use domain::items_total::FranchisesTotal;
use domain::items_total::GenresTotal;
use domain::items_total::PeopleTotal;
use domain::items_total::RolesTotal;
use domain::items_total::ThemesTotal;
use domain::items_total::UsersTotal;
use domain::score::Score;
use domain::slug::Slug;
use utoipa::openapi::OpenApi;
use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};

use crate::controllers::account_controller::LoginData;
use crate::controllers::account_controller::LoginReturnData;
use crate::controllers::account_controller::account_doc::AccountDoc;
use crate::controllers::book_controller::book_doc::BookDoc;
use crate::controllers::character_controller::character_doc::CharacterDoc;
use crate::controllers::franchise_controller::franchise_doc::FranchiseDoc;
use crate::controllers::genre_controller::genre_doc::GenreDoc;
use crate::controllers::person_controller::person_doc::PersonDoc;
use crate::controllers::role_controller::role_doc::RoleDoc;
use crate::controllers::theme_controller::theme_doc::ThemeDoc;
use crate::controllers::user_controller::user_doc::UserDoc;

#[derive(utoipa::OpenApi)]
#[openapi(info(title = "pileofmedia"),
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
    BookCharacter,
    User,
    InvolvedId,
    CreateAccount,
    CreateAccountData,
    CreateUser,
    CreateUserData,
    CreateImage,
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
    UsersTotal,
    FranchisesTotal,
    Language,
    CreateBook,
    CreateBookData,
    CreateBookTranslation,
    CreateCover,
    CreatePerson,
    CreatePersonData,
    CreatePersonTranslation,
    CreateGenre,
    CreateGenreTranslation,
    CreateTheme,
    CreateThemeTranslation,
    CreateRole,
    CreateRoleTranslation,
    CreateFranchise,
    CreateFranchiseTranslation,
    Slug,
    BookStatistic,
    Rating,
    GenreAvailableTranslations,
    GenreTranslation,
    ThemeAvailableTranslations,
    ThemeTranslation,
    CreateCharacter,
    CreateCharacterData,
    CreateCharacterTranslation,
    CharacterAvailableTranslations,
    CharacterTranslation,
    RoleTranslation,
    RoleAvailableTranslations,
    PersonTranslation,
    PersonAvailableTranslations,
    Involved,
    FranchiseTranslation,
    FranchiseAvailableTranslations,
    BookTranslation,
    BookAvailableTranslations,
    UserBook,
    Score,
    UserStatus,
    CreateUserBook
  )))]
pub(crate) struct ApiDoc;

pub struct JsonWebTokenParam;

impl utoipa::Modify for JsonWebTokenParam {
  fn modify(&self, openapi: &mut OpenApi) {
    if let Some(components) = openapi.components.as_mut() {
      components.add_security_scheme(
        "user_token",
        SecurityScheme::Http(
          HttpBuilder::new()
            .scheme(HttpAuthScheme::Bearer)
            .bearer_format("JWT")
            .build(),
        ),
      );
    }
  }
}
