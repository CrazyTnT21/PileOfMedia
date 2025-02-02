use axum::async_trait;
use axum::extract::{FromRequest, Multipart, Request};
use std::collections::HashMap;
use std::error::Error;

#[async_trait]
pub trait FromMultiPart {
  type Error: Error;
  async fn from_multi_part(multipart: Multipart) -> Result<Self, Self::Error>
  where
    Self: Sized;
}
pub struct MultiPartRequest<T>(pub T);

#[async_trait]
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
