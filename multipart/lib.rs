use std::error::Error;
use axum::async_trait;
use axum::extract::{FromRequest, Multipart, Request};

#[async_trait]
pub trait FromMultiPart {
    type Error: Error;
    async fn from_multi_part(multipart: Multipart) -> Result<Self, Self::Error>
    where
        Self: Sized;
}
pub struct MultiPartRequest<T>(pub T);

#[async_trait]
impl<T: FromMultiPart, S: Send + Sync> FromRequest<S> for MultiPartRequest<T>
{
    type Rejection = String;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let result = Multipart::from_request(req, state).await.map_err(|x| x.to_string())?;
        let item = T::from_multi_part(result).await.map_err(|x| x.to_string())?;
        let multipart = MultiPartRequest(item);
        Ok(multipart)
    }
}
pub use axum as axum;
