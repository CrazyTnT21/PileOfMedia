use crate::entities::book::Book;
use crate::entities::genre::Genre;

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
#[cfg_attr(feature = "utoipa", aliases(BooksTotal = ItemsTotal < Book >, GenresTotal = ItemsTotal < Genre >))]
pub struct ItemsTotal<T> {
  pub items: Vec<T>,
  pub total: usize,
}
