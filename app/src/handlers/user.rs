use crate::{
  middleware::auth,
  models::user::User,
  utils::{database::DataBase, response, response::JsonError},
};
use bcrypt::{hash, DEFAULT_COST};
use chrono::NaiveDate;
use garde::Validate;
use poem::{
  get,
  handler,
  http::StatusCode,
  post,
  web::{Data, Json},
  EndpointExt,
  Response,
  Route,
};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, Validate)]
struct UserCreate {
  #[garde(required, ascii, length(min = 3, max = 25))]
  name: Option<String>,
  #[garde(required, email)]
  email: Option<String>,
  #[garde(required, length(min = 6))]
  password: Option<String>,
  #[garde(
    required,
    pattern(r"([12]\d{3}-(0[1-9]|1[0-2])-(0[1-9]|[12]\d|3[01]))")
  )]
  born_date: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Validate)]
struct UserAddNotificationToken {
  #[garde(required)]
  notification_token: Option<String>,
}

#[handler]
async fn create(req: Json<UserCreate>, pool: Data<&DataBase>) -> Response {
  if let Err(e) = req.0.validate(&()) {
    return response::json(response::garde_error_to_json(e), StatusCode::BAD_REQUEST);
  }

  let password = hash(req.0.password.unwrap(), DEFAULT_COST).unwrap();
  let born_date = NaiveDate::parse_from_str(&req.0.born_date.unwrap(), "%Y-%m-%d").unwrap();
  let email = req.0.email.unwrap();

  let user = User::find_by_email(pool.clone(), &email).await;

  if user.is_ok() {
    return response::json(
      serde_json::json!({ "errors": vec![JsonError::new("email".to_string(), "arready taken".to_string())] }),
      StatusCode::BAD_REQUEST,
    );
  }

  let id = User::insert(
    pool.clone(),
    &req.0.name.unwrap(),
    &password,
    &email,
    &born_date.and_hms_opt(0, 0, 0).unwrap(),
  )
  .await
  .unwrap();

  response::json(
    serde_json::json! ({
      "user_id": id,
    }),
    StatusCode::CREATED,
  )
}

#[handler]
async fn get_authenticated_user(user: Data<&User>, pool: Data<&DataBase>) -> Response {
  let mut user = user.0.clone();

  let has_unviewed_notifications = User::get_notifications(pool.clone(), user.id)
    .await
    .unwrap()
    .iter()
    .any(|notification| !notification.viewed);

  response::json_ok(serde_json::json!({
    "user": {
      "id": user.id,
      "name": user.name,
      "email": user.email,
      "has_unviewed_notifications": has_unviewed_notifications,
      "born_date": user.born_date,
      "created_at": user.create_date,
    }
  }))
}

#[handler]
async fn add_notification_token(
  req: Json<UserAddNotificationToken>,
  user: Data<&User>,
  pool: Data<&DataBase>,
) -> Response {
  let mut user = user.0.clone();

  User::update_notification_token(pool.clone(), user.id, &req.0.notification_token.unwrap())
    .await
    .unwrap();

  response::json_ok(serde_json::json!({
    "message": "ok",
  }))
}

#[handler]
async fn get_notifications(user: Data<&User>, pool: Data<&DataBase>) -> Response {
  let user = user.0.clone();

  let notifications_db = User::get_notifications(pool.clone(), user.id)
    .await
    .unwrap();

  let notifications: Vec<serde_json::Value> = notifications_db
    .iter()
    .map(|notification| {
      serde_json::json!({
        "id": notification.id,
        "message": notification.message,
        "create_date": notification.create_date.format("%d/%m/%Y %H:%M:%S").to_string(),
      })
    })
    .collect();

  User::update_viewed_notifications(pool.clone(), user.id)
    .await
    .unwrap();

  response::json_ok(serde_json::json!({
    "notifications": notifications,
  }))
}

pub fn routes() -> Route {
  Route::new()
    .just_at(get(get_authenticated_user).around(auth::handle))
    .at("/register", post(create))
    .at("/notification", get(get_notifications).around(auth::handle))
    .at(
      "/notification-token",
      post(add_notification_token).around(auth::handle),
    )
}
