// use crate::entities::book::create_book_edition::CreateBookEditionData;
use crate::entities::image::create_image::CreateImage;
use crate::entities::involved::InvolvedId;
use crate::enums::language::Language;
use crate::slug::Slug;
use chrono::NaiveDate;
use std::collections::HashMap;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreateBook {
  pub book: CreateBookData,
  pub covers: Vec<CreateImage>,
  // pub images: Option<Vec<CreateImage>>,
  // pub editions: Vec<CreateBookEditionData>,
  // pub edition_covers: Vec<CreateImage>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreateBookData {
  pub slug: Slug,
  pub published: Option<NaiveDate>,
  pub franchise: Option<u32>,
  pub translations: HashMap<Language, CreateBookTranslation>,
  pub genres: Option<Vec<u32>>,
  pub themes: Option<Vec<u32>>,
  pub characters: Option<Vec<u32>>,
  pub involved: Option<Vec<InvolvedId>>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreateBookTranslation {
  pub title: String,
  pub description: Option<String>,
  pub cover: CreateCover,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub enum CreateCover {
  ImageIndex(usize),
  ReuseFromLanguage(Language),
}

#[cfg(feature = "axum-multipart")]
pub mod create_book_part {
  use crate::entities::book::create_book::{CreateBook, CreateBookData};
  // use crate::entities::book::create_book_edition::CreateBookEditionData;
  use crate::entities::image::create_image::CreateImage;
  use crate::vec_single::{Single, SingleVecError};
  use multipart::axum::extract::multipart::MultipartError;
  use multipart::axum::extract::Multipart;
  use multipart::{serialize_parts, FromMultiPart};
  use serde_json::from_slice;
  use std::error::Error;
  use std::fmt::{Display, Formatter};

  #[derive(Debug)]
  pub enum CreateBookPartError {
    BookMissing,
    MoreThanOneBook,
    OtherError(Box<dyn Error + Send>),
  }
  impl From<serde_json::Error> for CreateBookPartError {
    fn from(value: serde_json::Error) -> Self {
      CreateBookPartError::OtherError(Box::new(value))
    }
  }
  impl From<MultipartError> for CreateBookPartError {
    fn from(value: MultipartError) -> Self {
      CreateBookPartError::OtherError(Box::new(value))
    }
  }

  impl Display for CreateBookPartError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
      write!(
        f,
        "{}",
        match self {
          CreateBookPartError::BookMissing => "Book missing".to_string(),
          CreateBookPartError::OtherError(value) => value.to_string(),
          CreateBookPartError::MoreThanOneBook => "There is more than 1 book".to_string(),
        }
      )
    }
  }

  impl Error for CreateBookPartError {}

  #[async_trait::async_trait]
  impl FromMultiPart for CreateBook {
    type Error = CreateBookPartError;

    async fn from_multi_part(multipart: Multipart) -> Result<Self, Self::Error>
    where
      Self: Sized,
    {
      let mut parts = serialize_parts(multipart).await?;
      let book_bytes = parts
        .remove(&Some("book".to_string()))
        .ok_or_else(|| CreateBookPartError::BookMissing)?
        .single()
        .map_err(|x| match x {
          SingleVecError::NoItems => CreateBookPartError::BookMissing,
          SingleVecError::MoreThanOneItem(_) => CreateBookPartError::MoreThanOneBook,
        })?;

      let book: CreateBookData = from_slice(&book_bytes)?;
      // let editions = parts.remove(&Some("editions".to_string())).unwrap_or_else(Vec::new);
      // let editions: Result<Vec<CreateBookEditionData>, serde_json::Error> = editions
      //   .into_iter()
      //   .map(|x| {
      //     let edition: Result<CreateBookEditionData, serde_json::Error> = from_slice(&x);
      //     edition
      //   })
      //   .collect::<Vec<Result<CreateBookEditionData, serde_json::Error>>>()
      //   .into_iter()
      //   .collect();
      // let editions = editions?;

      let covers = parts.remove(&Some("covers".to_string())).unwrap_or_else(Vec::new);
      let covers: Vec<CreateImage> = covers.into_iter().map(|x| CreateImage(x.to_vec())).collect();

      // let images = parts.remove(&Some("images".to_string())).unwrap_or_else(Vec::new);
      // let images: Option<Vec<CreateImage>> = Some(images.into_iter().map(|x| CreateImage(x.to_vec())).collect());

      // let edition_covers = parts
      // .remove(&Some("edition_covers".to_string()))
      // .unwrap_or_else(Vec::new);
      // let edition_covers: Vec<CreateImage> = edition_covers.into_iter().map(|x| CreateImage(x.to_vec())).collect();
      Ok(CreateBook {
        book,
        covers,
        // images,
        // editions,
        // edition_covers,
      })
    }
  }
}
