use crate::utils::database::DataBase;
use chrono::{naive::serde::ts_seconds, NaiveDateTime};
use sqlx::{types::Uuid, Result};

#[derive(Debug, serde::Serialize, Clone)]
pub(crate) struct User {
  pub id: Uuid,
  pub name: String,
  pub email: String,
  pub password: String,
  #[serde(with = "ts_seconds")]
  pub born_date: NaiveDateTime,
  #[serde(with = "ts_seconds")]
  pub create_date: NaiveDateTime,
  pub notification_token: Option<String>,
}

#[derive(Debug, serde::Serialize, Clone)]
pub(crate) struct UserNotifications {
  pub id: i64,
  pub user_id: Uuid,
  pub message: String,
  pub viewed: bool,
  #[serde(with = "ts_seconds")]
  pub create_date: NaiveDateTime,
}

impl User {
  // pub fn new(id: i32, name: String, email: String, password: String, born_date: NaiveDateTime, create_date: NaiveDateTime) -> User {
  //   User {
  //     id,
  //     name,
  //     email,
  //     password,
  //     born_date,
  //     create_date
  //   }
  // }

  // pub async fn get_all(database: DataBase) -> Result<Vec<User>, Error> {
  //   return sqlx::query_as!(User, "SELECT * FROM users")
  //     .fetch_all(&database.pool)
  //     .await;
  // }

  pub async fn insert(
    database: DataBase,
    name: &String,
    password: &String,
    email: &String,
    born_date: &NaiveDateTime,
  ) -> Result<String> {
    let result = sqlx::query!(
      "INSERT INTO users (id, name, password, email, born_date) VALUES ($1, $2, $3, $4, $5) RETURNING id",
      Uuid::new_v4(),
      name,
      password,
      email,
      born_date
    )
    .fetch_one(&database.pool)
    .await
    .map_err(DataBase::database_error)?;

    Ok(result.id.to_string())
  }

  pub async fn find_by_email(database: DataBase, email: &String) -> Result<User> {
    Ok(
      sqlx::query_as!(User, "SELECT * FROM users WHERE email = $1 LIMIT 1", email)
        .fetch_one(&database.pool)
        .await
        .map_err(DataBase::database_error)?,
    )
  }

  pub(crate) async fn find_by_uuid(database: &DataBase, uid: Uuid) -> Result<User> {
    Ok(
      sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1 LIMIT 1", uid)
        .fetch_one(&database.pool)
        .await
        .map_err(DataBase::database_error)?,
    )
  }

  pub(crate) async fn update_notification_token(
    database: DataBase,
    uid: Uuid,
    token: &String,
  ) -> Result<()> {
    sqlx::query!(
      "UPDATE users SET notification_token = $1 WHERE id = $2",
      token,
      uid
    )
    .execute(&database.pool)
    .await
    .map_err(DataBase::database_error)?;

    Ok(())
  }

  pub(crate) async fn get_notifications(
    database: DataBase,
    uid: Uuid,
  ) -> Result<Vec<UserNotifications>> {
    Ok(sqlx::query_as!(
      UserNotifications,
      "SELECT id, user_id, message, viewed, create_date FROM user_notifications WHERE user_id = $1 ORDER BY create_date DESC LIMIT 20",
      uid
    )
    .fetch_all(&database.pool)
    .await
    .map_err(DataBase::database_error)?)
  }

  pub(crate) async fn update_viewed_notifications(database: DataBase, uid: Uuid) -> Result<()> {
    sqlx::query!(
      "UPDATE user_notifications SET viewed = true WHERE user_id = $1",
      uid
    )
    .execute(&database.pool)
    .await
    .map_err(DataBase::database_error)?;

    Ok(())
  }
}
