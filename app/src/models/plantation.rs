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
  pub planting_date: chrono::NaiveDateTime,
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
            plantations.planting_date,
            plantations.create_date,
            plantations.update_date,
            st_x(plantations.location::geometry) AS latitude,
            st_y(plantations.location::geometry) AS longitude
      FROM plantations
      WHERE user_id = $1
      ORDER BY plantations.create_date DESC
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
            plantations.planting_date,
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
    planting_date: chrono::NaiveDateTime,
  ) -> Result<Uuid> {
    let point = format!(
      "POINT ({} {})",
      latitude.to_string().replace(",", "."),
      longitude.to_string().replace(",", ".")
    );

    let result = sqlx::query!(
      "INSERT INTO plantations (id, user_id, culture_id, station_id, alias, location, area, planting_date) VALUES ($1, $2, $3, $4, $5, ST_PointFromText($6)::point, $7, $8) RETURNING id",
      Uuid::new_v4(),
      user_id,
      culture_id,
      station_id,
      alias,
      point,
      area,
      planting_date
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
    planting_date: chrono::NaiveDateTime,
  ) -> Result<()> {
    let point = format!(
      "POINT ({} {})",
      latitude.to_string().replace(",", "."),
      longitude.to_string().replace(",", ".")
    );

    sqlx::query!(
      "UPDATE plantations SET culture_id = $2, station_id = $3, alias = $4, location = ST_PointFromText($5)::point, area = $6, planting_date = $7 WHERE id = $1",
      id,
      culture_id,
      station_id,
      alias,
      point,
      area,
      planting_date
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

  pub(crate) async fn has_ocurrence_last_24h(db: &DataBase, id: Uuid) -> Result<bool> {
    let result = sqlx::query!(
      "SELECT EXISTS(SELECT 1 FROM plantation_pathogenic_occurrences WHERE plantation_id = $1 AND occurrence_date >= now() - interval '24 hours') AS has_ocurrence_last_24h",
      id
    )
    .fetch_one(&db.pool)
    .await
    .map_err(DataBase::database_error)?;

    Ok(result.has_ocurrence_last_24h.unwrap())
  }
}
