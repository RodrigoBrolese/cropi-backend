use crate::utils::database::DataBase;
use sqlx::{types::Uuid, Result};

#[derive(Debug, serde::Serialize, Clone)]
pub(crate) struct Plantation {
  pub id: Uuid,
  pub user_id: Uuid,
  pub culture_id: i64,
  pub station_id: Option<i64>,
  pub alias: Option<String>,
  pub latitude: Option<f64>,
  pub longitude: Option<f64>,
  pub area: f64,
  pub create_date: chrono::NaiveDateTime,
  pub update_date: Option<chrono::NaiveDateTime>,
}

impl Plantation {
  pub async fn all_by_user_id(db: &DataBase, user_id: Uuid) -> Vec<Plantation> {
    sqlx::query_as!(
      Plantation,
      "
      SELECT plantations.id,
            plantations.user_id,
            plantations.culture_id,
            plantations.station_id,
            plantations.alias,
            plantations.area,
            plantations.create_date,
            plantations.update_date,
            st_x(plantations.location::geometry) AS latitude,
            st_y(plantations.location::geometry) AS longitude
      FROM plantations
      WHERE user_id = $1
    ",
      user_id
    )
    .fetch_all(&db.pool)
    .await
    .unwrap()
  }

  pub async fn find_by_uuid(db: &DataBase, uuid: Uuid) -> Result<Plantation> {
    sqlx::query_as!(
      Plantation,
      "
      SELECT plantations.id,
            plantations.user_id,
            plantations.culture_id,
            plantations.station_id,
            plantations.alias,
            plantations.area,
            plantations.create_date,
            plantations.update_date,
            st_x(plantations.location::geometry) AS latitude,
            st_y(plantations.location::geometry) AS longitude
      FROM plantations
      WHERE plantations.id = $1
    ",
      uuid
    )
    .fetch_one(&db.pool)
    .await
  }

  pub async fn insert(
    db: &DataBase,
    user_id: Uuid,
    culture_id: i64,
    station_id: Option<i64>,
    alias: String,
    latitude: f64,
    longitude: f64,
    area: f64,
  ) -> Result<Uuid> {
    let point = format!(
      "POINT ({} {})",
      latitude.to_string().replace(",", "."),
      longitude.to_string().replace(",", ".")
    );

    let result = sqlx::query!(
      "INSERT INTO plantations (id, user_id, culture_id, station_id, alias, location, area) VALUES ($1, $2, $3, $4, $5, ST_PointFromText($6)::point, $7) RETURNING id",
      Uuid::new_v4(),
      user_id,
      culture_id,
      station_id,
      alias,
      point,
      area
    )
    .fetch_one(&db.pool)
    .await
    .map_err(DataBase::database_error)?;

    Ok(result.id)
  }

  pub async fn update(
    db: &DataBase,
    id: Uuid,
    culture_id: i64,
    station_id: Option<i64>,
    alias: String,
    latitude: f64,
    longitude: f64,
    area: f64,
  ) -> Result<()> {
    let point = format!(
      "POINT ({} {})",
      latitude.to_string().replace(",", "."),
      longitude.to_string().replace(",", ".")
    );

    sqlx::query!(
      "UPDATE plantations SET culture_id = $1, station_id = $2, alias = $3, location = ST_PointFromText($4)::point, area = $5 WHERE id = $6",
      culture_id,
      station_id,
      alias,
      point,
      area,
      id
    )
    .execute(&db.pool)
    .await
    .map_err(DataBase::database_error)?;

    Ok(())
  }

  pub async fn delete(db: &DataBase, id: Uuid) -> Result<()> {
    sqlx::query!("UPDATE plantations SET delete_at = now() WHERE id = $1", id)
      .execute(&db.pool)
      .await
      .map_err(DataBase::database_error)?;

    Ok(())
  }
}
