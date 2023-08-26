use crate::utils::database::DataBase;
use chrono::{naive::serde::ts_seconds, NaiveDateTime};
use sqlx::{types::Uuid, Error, Result};

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

  pub async fn get_all(database: DataBase) -> Result<Vec<User>, Error> {
    return sqlx::query_as!(User, "SELECT * FROM users")
      .fetch_all(&database.pool)
      .await;
  }

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

  pub(crate) async fn find_by_uuid(database: DataBase, uid: Uuid) -> Result<User> {
    Ok(
      sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1 LIMIT 1", uid)
        .fetch_one(&database.pool)
        .await
        .map_err(DataBase::database_error)?,
    )
  }
}
