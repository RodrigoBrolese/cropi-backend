use crate::{
  models::user::User,
  utils::{database::DataBase, jwt::decode},
};
use poem::{http::StatusCode, Endpoint, Error, Request, Result};
use std::str::FromStr;
use uuid::Uuid;

pub(crate) async fn handle<E: Endpoint>(
  next: E,
  mut req: Request,
) -> Result<<E as Endpoint>::Output> {
  let token = req
    .headers()
    .get("Authorization")
    .and_then(|value| value.to_str().ok())
    .unwrap_or("");

  if token.is_empty() {
    return Err(Error::from_status(StatusCode::UNAUTHORIZED));
  }

  let tokens = token.split_once(" ").unwrap_or(("", ""));

  if tokens.0 != "Bearer" {
    return Err(Error::from_status(StatusCode::UNAUTHORIZED));
  }

  let claims = decode(tokens.1.to_string());

  if claims.is_err() {
    return Err(Error::from_status(StatusCode::UNAUTHORIZED));
  }

  let user = User::find_by_uuid(
    &req.data::<DataBase>().unwrap().clone(),
    Uuid::from_str(claims.unwrap().claims.sub.as_str()).unwrap(),
  )
  .await;

  if user.is_err() {
    return Err(Error::from_status(StatusCode::UNAUTHORIZED));
  }

  req.set_data(user.unwrap());

  return next.call(req).await;
}
