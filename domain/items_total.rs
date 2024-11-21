use crate::entities::book::book_character::BookCharacter;
use crate::entities::book::book_involved::BookInvolved;
use crate::entities::book::Book;
use crate::entities::character::Character;
use crate::entities::franchise::Franchise;
use crate::entities::genre::Genre;
use crate::entities::person::Person;
use crate::entities::role::Role;
use crate::entities::theme::Theme;
use crate::entities::user::User;

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
  FranchisesTotal = ItemsTotal < Franchise >,
  UsersTotal = ItemsTotal < User >
))]
pub struct ItemsTotal<T> {
  pub items: Vec<T>,
  pub total: usize,
}
