use axum::extract::{FromRequest, Multipart, Request};
use std::collections::HashMap;
use std::error::Error;

pub trait FromMultiPart: Sized {
  type Error: Error + Send;
  fn from_multi_part(multipart: Multipart) -> impl std::future::Future<Output = Result<Self, Self::Error>> + Send;
}
pub struct MultiPartRequest<T>(pub T);

impl<T: FromMultiPart, S: Send + Sync> FromRequest<S> for MultiPartRequest<T> {
  type Rejection = String;

  async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
    let result = Multipart::from_request(req, state).await.map_err(|x| x.to_string())?;
    let item = T::from_multi_part(result).await.map_err(|x| x.to_string())?;
    let multipart = MultiPartRequest(item);
    Ok(multipart)
  }
}
pub async fn serialize_parts(mut multipart: Multipart) -> Result<HashMap<Option<String>, Vec<Bytes>>, MultipartError> {
  let mut result: HashMap<Option<String>, Vec<Bytes>> = HashMap::new();
  while let Some(value) = multipart.next_field().await? {
    let previous = result.get_mut(&value.name().map(std::string::ToString::to_string));
    if let Some(set) = previous {
      set.push(value.bytes().await?);
      continue;
    }
    result.insert(
      value.name().map(std::string::ToString::to_string),
      vec![value.bytes().await?],
    );
  }
  Ok(result)
}
pub use axum;
use axum::body::Bytes;
use axum::extract::multipart::MultipartError;
