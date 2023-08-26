use crate::{
  models::user::User,
  utils::{database::DataBase, jwt, response},
};
use garde::Validate;
use poem::{
  handler,
  http::StatusCode,
  post,
  web::{Data, Json},
  Response,
  Route,
};

#[derive(Debug, serde::Deserialize, Clone, Validate)]
struct Login {
  #[garde(required, email)]
  email: Option<String>,
  #[garde(required, length(min = 6))]
  password: Option<String>,
}

#[handler]
async fn login(req: Json<Login>, pool: Data<&DataBase>) -> Response {
  if let Err(e) = req.0.validate(&()) {
    return response::json(response::garde_error_to_json(e), StatusCode::BAD_REQUEST);
  }

  let email = req.0.email.unwrap();

  let user_result = User::find_by_email(pool.clone(), &email).await;

  if user_result.is_err() {
    return response::json(
      serde_json::json!({"errors": vec![response::JsonError::new("user".to_string(), "Not found".to_string())]}),
      StatusCode::BAD_REQUEST,
    );
  }

  let user = user_result.unwrap();

  if !bcrypt::verify(req.0.password.unwrap(), &user.password).unwrap_or(false) {
    return response::json(
      serde_json::json!({"errors": vec![response::JsonError::new("user".to_string(), "Not found".to_string())]}),
      StatusCode::BAD_REQUEST,
    );
  }

  response::json_ok(serde_json::json!({
    "user_id": user.id,
    "name": user.name,
    "token": jwt::encode(user.id.to_string())
  }))
}

pub(crate) fn routes() -> Route {
  Route::new().at("/", post(login))
}
