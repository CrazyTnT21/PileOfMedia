use std::collections::HashMap;

use chrono::NaiveDate;

use crate::entities::book::book_involved::InvolvedId;
use crate::entities::image::create_image::CreateImage;
use crate::enums::language::Language;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct CreateBook {
  pub chapters: Option<u16>,
  pub pages: Option<u16>,
  pub words: Option<u32>,
  pub published: Option<NaiveDate>,
  pub franchise: Option<u32>,
  pub translations: HashMap<Language, CreateBookTranslation>,
  pub genres: Vec<u32>,
  pub themes: Vec<u32>,
  pub characters: Vec<u32>,
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
  Image(CreateImage),
  ReuseFromLanguage(Language),
}

#[cfg(feature = "axum-multipart")]
pub mod create_book_part {
  use std::collections::HashMap;
  use std::error::Error;
  use std::fmt::{Display, Formatter};
  use std::str::FromStr;

  use crate::entities::book::create_book::{CreateBook, CreateCover};
  use crate::enums::language::{Language, LanguageError};

  #[derive(Debug)]
  pub enum CreateBookPart {
    Book,
    Cover(Language),
    // Image,
  }

  #[derive(Debug)]
  pub enum CreateBookPartError {
    MissingPart,
    InvalidFormat,
    BookMissing,
    NonExistentTranslation(Language),
    UnknownPart(String),
    InvalidVariant,
    CoverLanguageMissing,
    CoverLanguageError(LanguageError),
    OtherError(Box<dyn Error + Send>),
  }


  impl Display for CreateBookPartError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
      write!(f, "{}",
             match self {
               CreateBookPartError::InvalidFormat => "Invalid part format".to_string(),
               CreateBookPartError::UnknownPart(value) => format!("Unknown part '{}'", value),
               CreateBookPartError::InvalidVariant => "Invalid variant".to_string(),
               CreateBookPartError::CoverLanguageError(value) => format!("Cover error: {}", value),
               CreateBookPartError::CoverLanguageMissing => "Cover X-Language header missing".to_string(),
               CreateBookPartError::BookMissing => "Book missing".to_string(),
               CreateBookPartError::MissingPart => "Missing part value".to_string(),
               CreateBookPartError::OtherError(value) => value.to_string(),
               CreateBookPartError::NonExistentTranslation(language) => format!("language '{language}' has no translation")
             })
    }
  }
  impl Error for CreateBookPartError {}

  impl CreateBookPart {
    fn from_header(name: &str, headers: &multipart::axum::http::HeaderMap) -> Result<Self, CreateBookPartError> {
      if name.is_empty() {
        return Err(CreateBookPartError::InvalidFormat);
      }
      let language = || headers.get("X-language")
          .ok_or(CreateBookPartError::CoverLanguageMissing)?.to_str()
          .map_err(|x| CreateBookPartError::OtherError(Box::new(x)));

      let result = match name.to_lowercase().as_str() {
        "book" => CreateBookPart::Book,
        "cover" => CreateBookPart::Cover(Language::from_str(language()?).map_err(|x| CreateBookPartError::CoverLanguageError(x))?),
        // "image" => CreateBookPart::Image,
        _ => Err(CreateBookPartError::UnknownPart(name.to_string()))?
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
      let mut book: Option<CreateBook> = None;
      let mut covers = HashMap::new();
      // let mut images = vec![];
      while let Some(field) = multipart.next_field().await.map_err(|x| CreateBookPartError::OtherError(Box::new(x)))? {
        let part = CreateBookPart::from_header(field.name().ok_or(CreateBookPartError::MissingPart)?, field.headers())?;
        match part {
          CreateBookPart::Book => {
            let create_book = serde_json::from_slice::<CreateBook>(&field.bytes()
                .await
                .map_err(|x| CreateBookPartError::OtherError(Box::new(x)))?)
                .map_err(|x| CreateBookPartError::OtherError(Box::new(x)))?;
            book = Some(create_book);
          }
          CreateBookPart::Cover(language) => {
            covers.insert(language, field.bytes().await.map_err(|x| CreateBookPartError::OtherError(Box::new(x)))?);
          }
          // CreateBookPart::Image => { images.push(a.bytes().await.map_err(|x| CreateBookPartError::OtherError(Box::new(x)))?); }
        }
      }

      let mut book = book.ok_or(CreateBookPartError::BookMissing)?;

      for (language, bytes) in covers {
        match &mut book.translations.get_mut(&language)
            .ok_or(CreateBookPartError::NonExistentTranslation(language))?.cover {
          CreateCover::Image(ref mut image) => image.0 = bytes.to_vec(),
          _ => Err(CreateBookPartError::InvalidVariant)?
        }
      }
      Ok(book)
    }
  }
}