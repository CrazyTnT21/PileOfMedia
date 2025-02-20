use crate::controllers::convert_error;
use crate::extractors::headers::authorization::JWTAuthorization;
use axum::http::StatusCode;
use chrono::Utc;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

pub fn parse_token(auth: JWTAuthorization, secret: &str) -> Result<Claim, (StatusCode, String)> {
  let claim = jsonwebtoken::decode::<Claim>(
    &auth.token,
    &DecodingKey::from_secret(secret.as_bytes()),
    &Validation::default(),
  )
  .map_err(|_| (StatusCode::FORBIDDEN, "Invalid JWT".to_string()))?;
  Ok(claim.claims)
}
pub fn encode_token(
  user_id: u32,
  subject: String,
  timestamp: usize,
  secret: &str,
) -> Result<String, (StatusCode, String)> {
  let claim = create_claim(subject, user_id, timestamp);
  let token = create_token(claim, secret.as_bytes())?;
  Ok(token)
}

fn create_token(claim: Claim, secret: &[u8]) -> Result<String, (StatusCode, String)> {
  let key = EncodingKey::from_secret(secret);
  let header = Header::default();

  jsonwebtoken::encode(&header, &claim, &key).map_err(convert_error)
}

fn create_claim(subject: String, user_id: u32, exp: usize) -> Claim {
  Claim {
    user_id,
    sub: subject,
    iss: "PileOfMedia".to_string(),
    exp,
    iat: usize::try_from(Utc::now().timestamp()).unwrap(),
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claim {
  pub user_id: u32,
  pub sub: String,
  pub exp: usize,
  pub iat: usize,
  pub iss: String,
}
