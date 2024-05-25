use crate::entities::book::Book;

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[cfg_attr(feature = "utoipa", aliases(BooksTotal = ItemsTotal<Book>))]
pub struct ItemsTotal<T> {
  pub items: Vec<T>,
  pub total: usize,
}
