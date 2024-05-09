use std::error::Error;
use std::str::FromStr;

use tokio_postgres::types::{FromSql, IsNull, Kind, to_sql_checked, ToSql, Type};
use tokio_postgres::types::private::BytesMut;

use domain::entities::image::image::ImageExtension;
use from_row::from_row_impl;
use from_row::FromRow;
use from_row::FromRowOption;

use crate::{convert, enum_from_sql};

#[derive(Debug)]
pub enum DbImageExtension {
  JPG,
  JPEG,
  PNG,
  GIF,
}
from_row_impl!(DbImageExtension);
convert!(DbImageExtension,ImageExtension, JPG,JPEG,PNG,GIF);
enum_from_sql!(DbImageExtension,"imageextension");

impl DbImageExtension {
  pub fn db_name(&self) -> &'static str {
    match self {
      DbImageExtension::JPG => "JPG",
      DbImageExtension::JPEG => "JPEG",
      DbImageExtension::PNG => "PNG",
      DbImageExtension::GIF => "GIF"
    }
  }
}

impl FromStr for DbImageExtension{
  type Err = <ImageExtension as FromStr>::Err;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    ImageExtension::from_str(s).and_then(|x| Ok(DbImageExtension::from(x)))
  }
}

impl ToSql for DbImageExtension {
  fn to_sql(&self, _ty: &Type, out: &mut BytesMut) -> Result<IsNull, Box<dyn Error + Sync + Send>> where Self: Sized {
    out.extend_from_slice(&self.db_name().bytes().collect::<Vec<u8>>());
    Ok(IsNull::No)
  }

  fn accepts(ty: &Type) -> bool where Self: Sized {
    <DbImageExtension as FromSql>::accepts(ty)
  }
  to_sql_checked!();
}
