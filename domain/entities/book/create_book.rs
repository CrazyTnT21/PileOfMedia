use std::collections::HashMap;

use chrono::NaiveDate;

use crate::entities::book::book_involved::InvolvedId;
use crate::entities::image::create_image::CreateImage;
use crate::enums::language::Language;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreateBook {
  pub book: CreateBookData,
  pub covers: Vec<CreateImage>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreateBookData {
  pub chapters: Option<u16>,
  pub pages: Option<u16>,
  pub words: Option<u32>,
  pub published: Option<NaiveDate>,
  pub franchise: Option<u32>,
  pub translations: HashMap<Language, CreateBookTranslation>,
  #[cfg_attr(feature = "serde", serde(default))]
  pub genres: Vec<u32>,
  #[cfg_attr(feature = "serde", serde(default))]
  pub themes: Vec<u32>,
  #[cfg_attr(feature = "serde", serde(default))]
  pub characters: Vec<u32>,
  #[cfg_attr(feature = "serde", serde(default))]
  pub involved: Vec<InvolvedId>,
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
  use std::error::Error;
  use std::fmt::{Display, Formatter};

  use crate::entities::book::create_book::{CreateBook, CreateBookData};
  use crate::entities::image::create_image::CreateImage;

  #[derive(Debug)]
  pub enum CreateBookPart {
    Book,
    Cover,
    // Image,
  }

  #[derive(Debug)]
  pub enum CreateBookPartError {
    MissingPart,
    InvalidFormat,
    BookMissing,
    UnknownPart(String),
    OtherError(Box<dyn Error + Send>),
  }

  impl Display for CreateBookPartError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
      write!(
        f,
        "{}",
        match self {
          CreateBookPartError::InvalidFormat => "Invalid part format".to_string(),
          CreateBookPartError::UnknownPart(value) => format!("Unknown part '{}'", value),
          CreateBookPartError::BookMissing => "Book missing".to_string(),
          CreateBookPartError::MissingPart => "Missing part value".to_string(),
          CreateBookPartError::OtherError(value) => value.to_string(),
        }
      )
    }
  }

  impl Error for CreateBookPartError {}

  impl CreateBookPart {
    fn from_header(name: &str) -> Result<Self, CreateBookPartError> {
      if name.is_empty() {
        return Err(CreateBookPartError::InvalidFormat);
      }

      let result = match name.to_lowercase().as_str() {
        "book" => CreateBookPart::Book,
        "covers" => CreateBookPart::Cover,
        // "image" => CreateBookPart::Image,
        _ => Err(CreateBookPartError::UnknownPart(name.to_string()))?,
      };
      Ok(result)
    }
  }

  #[async_trait::async_trait]
  impl multipart::FromMultiPart for CreateBook {
    type Error = CreateBookPartError;

    async fn from_multi_part(mut multipart: multipart::axum::extract::Multipart) -> Result<Self, Self::Error>
    where
      Self: Sized,
    {
      let mut book: Option<CreateBookData> = None;
      let mut covers: Vec<CreateImage> = vec![];
      // let mut images = vec![];
      while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|x| CreateBookPartError::OtherError(Box::new(x)))?
      {
        let part = CreateBookPart::from_header(field.name().ok_or(CreateBookPartError::MissingPart)?)?;
        match part {
          CreateBookPart::Book => {
            let create_book = serde_json::from_slice::<CreateBookData>(
              &field
                .bytes()
                .await
                .map_err(|x| CreateBookPartError::OtherError(Box::new(x)))?,
            )
            .map_err(|x| CreateBookPartError::OtherError(Box::new(x)))?;
            book = Some(create_book);
          }
          CreateBookPart::Cover => {
            covers.push(CreateImage(
              field
                .bytes()
                .await
                .map_err(|x| CreateBookPartError::OtherError(Box::new(x)))?
                .to_vec(),
            ));
          } // CreateBookPart::Image => { images.push(a.bytes().await.map_err(|x| CreateBookPartError::OtherError(Box::new(x)))?); }
        }
      }

      let book = CreateBook {
        book: book.ok_or(CreateBookPartError::BookMissing)?,
        covers,
      };

      Ok(book)
    }
  }
}
