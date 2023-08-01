use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Claims {
  pub sub: String,
  pub company: String,
  pub exp: usize,
}

pub(crate) fn encode(sub: String) -> String {
  let secret = std::env::var("APP_SECRET").unwrap();

  let claims = Claims {
    sub,
    company: "Cropi".to_string(),
    exp: (Utc::now() + Duration::days(182)).timestamp() as usize,
  };

  let token = jsonwebtoken::encode(
    &Header::default(),
    &claims,
    &EncodingKey::from_secret(secret.as_ref()),
  );

  if token.is_err() {
    panic!()
  }

  token.unwrap()
}

pub(crate) fn decode(
  jwt: String,
) -> Result<jsonwebtoken::TokenData<Claims>, jsonwebtoken::errors::Error> {
  let secret = std::env::var("APP_SECRET").unwrap();

  jsonwebtoken::decode::<Claims>(
    &jwt,
    &DecodingKey::from_secret(secret.as_ref()),
    &Validation::default(),
  )
}
