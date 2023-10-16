use crate::utils::database::DataBase;
use chrono::Utc;
use reqwest::Client;

#[derive(Debug, serde::Serialize, Clone)]
pub(crate) struct Station {
  pub id: i64,
  pub status: bool,
  pub inmet_code: Option<String>,
}

#[derive(Debug, serde::Serialize, Clone)]

pub(crate) struct PathogenicCulture {
  pub(crate) id: i64,
  pub(crate) name: String,
  pub(crate) scientific_name: String,
  pub(crate) culture_id: i64,
  pub(crate) culture_name: String,
  pub(crate) culture_scientific_name: String,
}

pub(crate) async fn handler(
  client: &fantoccini::Client,
  pathogenic_id: String,
) -> Result<(), fantoccini::error::CmdError> {
  let db: DataBase = DataBase::new().await;

  let pathogenic = sqlx::query_as!(
    PathogenicCulture,
    "SELECT p.id,
          p.name,
          p.scientific_name,
          c.id              AS culture_id,
          c.name            AS culture_name,
          c.scientific_name AS culture_scientific_name
      FROM pathogenics p
            JOIN pathogenic_cultures pc ON pc.pathogenic_id = p.id
            JOIN cultures c ON c.id = pc.culture_id
      WHERE p.id = $1
      LIMIT 1",
    pathogenic_id.parse::<i64>().unwrap()
  )
  .fetch_one(&db.pool)
  .await
  .unwrap();

  let stations = sqlx::query_as!(
    Station,
    "SELECT s.id, s.status, s.inmet_code
    FROM stations s
        JOIN plantations p ON s.id = p.station_id
    WHERE s.status = TRUE
      AND s.inmet_code IS NOT NULL
      AND p.culture_id = $1
    GROUP BY s.id",
    pathogenic.culture_id
  )
  .fetch_all(&db.pool)
  .await
  .unwrap();

  let google_jwt_token = crate::utils::google_jwt::get_firebase_jwt().await;

  for station in stations {
    let stations_data = crate::scrapers::inmet_station_data::get_station_data(
      &client,
      station.inmet_code.unwrap(),
      Utc::now() - chrono::Duration::days(1),
    )
    .await;

    let hours_temperature = stations_data
      .iter()
      .filter(|data| data.temperature[0] > 17.0 && data.temperature[0] < 24.0)
      .count();

    let hours_humidity = stations_data
      .iter()
      .filter(|data| data.humidity[0] > 90.0)
      .count();

    if hours_temperature > 12 && hours_humidity > 12 {
      let users = sqlx::query!(
        "SELECT u.notification_token, u.id
        FROM stations s
        JOIN plantations p ON s.id = p.station_id
        JOIN users u ON u.id = p.user_id
        WHERE s.id = $1
          AND p.culture_id = $2
        GROUP BY u.id",
        station.id,
        pathogenic.culture_id
      )
      .fetch_all(&db.pool)
      .await
      .unwrap();

      for user in users {
        if user.notification_token.is_none() {
          continue;
        }

        let message = format!(
          "Detectamos que há probabilidade de {} em uma ou mais plantações de {}.",
          pathogenic.name, pathogenic.culture_name
        );

        let body = serde_json::json!({
          "message": {
            "token": user.notification_token.unwrap(),
            "notification": {
              "title": "ALERTA: Probabilidade de ocorrência",
              "body": message,
            },
          }
        });

        sqlx::query!(
          "INSERT INTO user_notifications (user_id, message) VALUES ($1, $2)",
          user.id,
          message
        )
        .execute(&db.pool)
        .await
        .unwrap();

        let _ = Client::new()
          .post("https://fcm.googleapis.com/v1/projects/cropi-399723/messages:send")
          .header("Content-Type", "application/json")
          .header("Authorization", format!("Bearer {}", google_jwt_token))
          .body(body.to_string())
          .send()
          .await;
      }
    }
  }

  Ok(())
}
