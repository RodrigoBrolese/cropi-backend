use crate::utils::database::DataBase;
use sqlx::Result;

#[derive(Debug, serde::Serialize, Clone)]
pub(crate) struct Culture {
  pub id: i64,
  pub name: String,
  pub scientific_name: String,
  pub description: Option<String>,
  pub create_date: chrono::NaiveDateTime,
}

impl Culture {
  pub(crate) async fn find_by_id(db: &DataBase, id: i64) -> Result<Culture> {
    sqlx::query_as!(
      Culture,
      "
            SELECT id,
                   name,
                   scientific_name,
                   description,
                   create_date
            FROM cultures
            WHERE id = $1
        ",
      id
    )
    .fetch_one(&db.pool)
    .await
  }
}
