use crate::utils::database::DataBase;
use chrono::NaiveDateTime;
use sqlx::Result;

#[derive(Debug, serde::Serialize, Clone)]

pub(crate) struct Pathogenic {
  pub(crate) id: i64,
  pub(crate) name: String,
  pub(crate) scientific_name: String,
  pub(crate) description: Option<String>,
  pub(crate) create_date: Option<NaiveDateTime>,
}

impl Pathogenic {
  //   pub(crate) async fn insert(
  //     database: DataBase,
  //     name: &String,
  //     scientific_name: &String,
  //     description: &Option<String>,
  //   ) -> Result<String> {
  //     let result = sqlx::query!(
  //             "INSERT INTO pathogenics (name, scientific_name, description) VALUES ($1, $2, $3) RETURNING id",
  //             name,
  //             scientific_name,
  //             description.as_ref()
  //         )
  //         .fetch_one(&database.pool)
  //         .await
  //         .map_err(DataBase::database_error)?;

  //     Ok(result.id.to_string())
  //   }

  pub(crate) async fn find_by_id(database: &DataBase, id: &i64) -> Result<Pathogenic> {
    Ok(
      sqlx::query_as!(
        Pathogenic,
        "SELECT * FROM pathogenics WHERE id = $1 LIMIT 1",
        id
      )
      .fetch_one(&database.pool)
      .await
      .map_err(DataBase::database_error)?,
    )
  }
}
