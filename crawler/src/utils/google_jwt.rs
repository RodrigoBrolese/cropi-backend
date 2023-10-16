use serde::{Deserialize, Serialize};
use yup_oauth2::ServiceAccountAuthenticator;

#[derive(Debug, Serialize, Deserialize)]
struct PayloadBody {
  iss: String,
  sub: String,
  aud: String,
  iat: usize,
  exp: usize,
  uid: String,
}

pub(crate) async fn get_firebase_jwt() -> String {
  let secret = yup_oauth2::read_service_account_key("assets/firebase-config.json")
    .await
    .expect("assets/firebase-config.json");

  let auth = ServiceAccountAuthenticator::builder(secret)
    .build()
    .await
    .unwrap();

  let scopes = &["https://www.googleapis.com/auth/firebase.messaging"];
  match auth.token(scopes).await {
    Ok(access_token) => access_token.token().unwrap_or("").to_string(),
    Err(_e) => "".to_string(),
  }
}
