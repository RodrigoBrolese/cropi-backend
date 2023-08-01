use sqlx::{Error, PgPool};

#[derive(Clone)]
pub struct DataBase {
  pub pool: PgPool,
}

impl DataBase {
  pub async fn new() -> Self {
    Self {
      pool: PgPool::connect(std::env::var("DATABASE_URL").unwrap().as_str())
        .await
        .expect("Error connecting to database"),
    }
  }

  pub fn database_error(err: Error) -> Error {
    tracing::error!("Failed to execute query: {:?}", err);
    err
  }
}
