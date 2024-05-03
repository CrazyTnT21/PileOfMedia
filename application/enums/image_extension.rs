use from_row::FromRowOption;
use from_row::FromRow;
use std::error::Error;
use std::str::FromStr;

use tokio_postgres::types::{FromSql, Kind, Type};

use domain::entities::image::image::ImageExtension;
use from_row::from_row_impl;

use crate::convert;

#[derive(Debug)]
pub enum DbImageExtension {
  JPG,
  JPEG,
  PNG,
  GIF,
}
from_row_impl!(DbImageExtension);
convert!(DbImageExtension,ImageExtension, JPG,JPEG,PNG,GIF);

impl<'a> FromSql<'a> for DbImageExtension {
  fn from_sql(_ty: &Type, raw: &'a [u8]) -> Result<Self, Box<dyn Error + Sync + Send>> {
    let extension = ImageExtension::from_str(std::str::from_utf8(raw)?)?;
    Ok(extension.into())
  }

  fn accepts(ty: &Type) -> bool {
    if ty.name() != "imageextension" {
      return false;
    }
    let Kind::Enum(value) = ty.kind() else {
      return false;
    };
    for x in value {
      match ImageExtension::from_str(x) {
        Ok(_) => {}
        Err(_error) => {
          return false;
        }
      }
    }

    true
  }
}

