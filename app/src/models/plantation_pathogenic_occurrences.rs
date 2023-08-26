use crate::utils::database::DataBase;
use chrono::NaiveDateTime;
use sqlx::Result;
use uuid::Uuid;

#[derive(Debug, serde::Serialize, Clone)]
pub(crate) struct PlantationPathogenicOccurrences {
  pub id: Uuid,
  pub user_id: Uuid,
  pub plantation_id: Uuid,
  pub pathogenic_id: i64,
  pub image: Option<String>,
  pub occurrence_date: NaiveDateTime,
  pub temperature: Option<f64>,
  pub humidity: Option<f64>,
  pub create_date: NaiveDateTime,
  pub update_date: Option<NaiveDateTime>,
}

impl PlantationPathogenicOccurrences {
  pub(crate) async fn get_by_plantation_id(
    db: &DataBase,
    plantation_id: Uuid,
  ) -> Result<Vec<PlantationPathogenicOccurrences>> {
    sqlx::query_as!(
      PlantationPathogenicOccurrences,
      "
            SELECT id,
                user_id,
                plantation_id,
                pathogenic_id,
                image,
                occurrence_date,
                temperature,
                humidity,
                create_date,
                update_date
            FROM plantation_pathogenic_occurrences
            WHERE plantation_id = $1
            ",
      plantation_id
    )
    .fetch_all(&db.pool)
    .await
  }

  pub(crate) async fn insert(
    db: &DataBase,
    user_id: Uuid,
    plantation_id: Uuid,
    pathogenic_id: i64,
    image: Option<String>,
    occurrence_date: NaiveDateTime,
    temperature: Option<f64>,
    humidity: Option<f64>,
  ) -> Result<PlantationPathogenicOccurrences> {
    sqlx::query_as!(
      PlantationPathogenicOccurrences,
      "
            INSERT INTO plantation_pathogenic_occurrences (id, user_id, plantation_id, pathogenic_id, image, occurrence_date, temperature, humidity)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING id, user_id, plantation_id, pathogenic_id, image, occurrence_date, temperature, humidity, create_date, update_date
            ",
      Uuid::new_v4(),
      user_id,
      plantation_id,
      pathogenic_id,
      image,
      occurrence_date,
      temperature,
      humidity
    )
    .fetch_one(&db.pool)
    .await
  }

  pub(crate) async fn find_by_id(
    db: &DataBase,
    id: Uuid,
  ) -> Result<PlantationPathogenicOccurrences> {
    sqlx::query_as!(
      PlantationPathogenicOccurrences,
      "
              SELECT id,
                  user_id,
                  plantation_id,
                  pathogenic_id,
                  image,
                  occurrence_date,
                  temperature,
                  humidity,
                  create_date,
                  update_date
              FROM plantation_pathogenic_occurrences
              WHERE id = $1
              ",
      id
    )
    .fetch_one(&db.pool)
    .await
  }

  pub(crate) async fn add_image_by_id(
    db: &DataBase,
    id: Uuid,
    image: String,
  ) -> Result<PlantationPathogenicOccurrences> {
    sqlx::query_as!(
      PlantationPathogenicOccurrences,
      "
              UPDATE plantation_pathogenic_occurrences
              SET image = $1
              WHERE id = $2
              RETURNING id,
                  user_id,
                  plantation_id,
                  pathogenic_id,
                  image,
                  occurrence_date,
                  temperature,
                  humidity,
                  create_date,
                  update_date
              ",
      image,
      id
    )
    .fetch_one(&db.pool)
    .await
  }
}
