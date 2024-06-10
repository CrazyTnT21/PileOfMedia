use crate::entities::user::User;
use crate::entities::role::Role;
use crate::entities::book::book_involved::BookInvolved;
use crate::entities::character::Character;
use crate::entities::book::Book;
use crate::entities::genre::Genre;
use crate::entities::theme::Theme;
use crate::entities::person::Person;
use crate::entities::book::book_character::BookCharacter;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[cfg_attr(feature = "utoipa", aliases(
BooksTotal = ItemsTotal < Book >,
GenresTotal = ItemsTotal < Genre >,
ThemesTotal = ItemsTotal < Theme >,
PeopleTotal = ItemsTotal < Person >,
CharactersTotal = ItemsTotal < Character >,
RolesTotal = ItemsTotal < Role >,
BookInvolvedTotal = ItemsTotal < BookInvolved >,
BookCharactersTotal = ItemsTotal < BookCharacter >,
UsersTotal = ItemsTotal < User >
))]
pub struct ItemsTotal<T> {
  pub items: Vec<T>,
  pub total: usize,
}
