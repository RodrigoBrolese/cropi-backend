use crate::utils::database::DataBase;
use sqlx::Result;

#[derive(Debug, serde::Serialize, Clone)]
pub(crate) struct Station {
  pub id: i64,
  pub city: String,
  pub uf: String,
  pub latitude: Option<f64>,
  pub longitude: Option<f64>,
  pub status: bool,
  pub inmet_code: Option<String>,
  pub create_date: chrono::NaiveDateTime,
  pub update_date: Option<chrono::NaiveDateTime>,
}

impl Station {
  pub(crate) async fn find_by_id(db: &DataBase, id: i64) -> Station {
    sqlx::query_as!(
      Station,
      "
      SELECT id,
            city,
            uf,
            st_x(stations.location::geometry) AS latitude,
            st_y(stations.location::geometry) AS longitude,
            status,
            inmet_code,
            create_date,
            update_date
      FROM stations
      WHERE id = $1;
        ",
      id
    )
    .fetch_one(&db.pool)
    .await
    .unwrap()
  }

  pub(crate) async fn find_closest_by_latitude_longitude(
    db: &DataBase,
    latitude: f64,
    longitude: f64,
  ) -> Result<Station> {
    sqlx::query_as!(
      Station,
      "
      SELECT id,
            city,
            uf,
            st_x(stations.location::geometry) AS latitude,
            st_y(stations.location::geometry) AS longitude,
            status,
            inmet_code,
            create_date,
            update_date
      FROM stations
      ORDER BY st_distance(st_makepoint($1, $2)::geometry, location::geometry)
      LIMIT 1
        ",
      latitude,
      longitude
    )
    .fetch_one(&db.pool)
    .await
  }
}
