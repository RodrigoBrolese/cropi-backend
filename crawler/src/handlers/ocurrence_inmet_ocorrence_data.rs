use crate::{scrapers::inmet_station_data::get_station_data, utils::database::DataBase};
use chrono::NaiveDateTime;
use fantoccini::error::CmdError;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone)]
struct Ocurrence {
  pub id: Uuid,
  pub station_immet_code: Option<String>,
  pub occurrence_date: NaiveDateTime,
  // pub temperature: Option<f64>,
  // pub humidity: Option<f64>,
  // pub update_date: Option<NaiveDateTime>,
}

pub(crate) async fn handler(
  client: &fantoccini::Client,
  ocurrence_id: String,
) -> Result<(), CmdError> {
  let db: DataBase = DataBase::new().await;

  let ocurrence = sqlx::query_as!(
    Ocurrence,
    "SELECT ppo.id, s.inmet_code AS station_immet_code, occurrence_date
    FROM plantation_pathogenic_occurrences AS ppo
             JOIN plantations p ON p.id = ppo.plantation_id
             JOIN stations s ON s.id = p.station_id
    WHERE ppo.id = $1
      AND s.inmet_code IS NOT NULL
      AND (SELECT COUNT(*)
           FROM plantation_pathogenic_occurrences_temperatures
           WHERE plantation_pathogenic_occurrence_id = ppo.id) = 0
      AND (SELECT COUNT(*)
            FROM plantation_pathogenic_occurrences_humidities
            WHERE plantation_pathogenic_occurrence_id = ppo.id) = 0
    LIMIT 1",
    Uuid::parse_str(&ocurrence_id).unwrap()
  )
  .fetch_one(&db.pool)
  .await
  .map_err(DataBase::database_error)
  .unwrap();

  let stations_data = get_station_data(
    &client,
    ocurrence.station_immet_code.unwrap(),
    (ocurrence.occurrence_date - chrono::Duration::days(12)).and_utc(),
  )
  .await;

  let mut temperatures: HashMap<String, Vec<String>> = HashMap::new();
  let mut humidities: HashMap<String, Vec<String>> = HashMap::new();

  for data in stations_data {
    if data.date.timestamp() > ocurrence.occurrence_date.timestamp() {
      continue;
    }

    // group by day and temperature

    let index_temperature = format!(
      "{}|{}",
      data.date.format("%Y-%m-%d").to_string(),
      data.temperature[0].floor().to_string()
    );

    let index_humidity = format!(
      "{}|{}",
      data.date.format("%Y-%m-%d").to_string(),
      data.humidity[0].to_string()
    );

    if temperatures.contains_key(&index_temperature) {
      temperatures
        .get_mut(&index_temperature)
        .unwrap()
        .push(data.date.format("%H:%M").to_string());
    } else {
      temperatures.insert(
        index_temperature,
        vec![data.date.format("%H:%M").to_string()],
      );
    }

    if humidities.contains_key(&index_humidity) {
      humidities
        .get_mut(&index_humidity)
        .unwrap()
        .push(data.date.format("%H:%M").to_string());
    } else {
      humidities.insert(index_humidity, vec![data.date.format("%H:%M").to_string()]);
    }
  }

  for temperature in temperatures {
    let temperature_values: Vec<&str> = temperature.0.split("|").collect();
    let date = temperature_values[0];
    let temperature_celcius = temperature_values[1];

    let temp = temperature.1.len();

    sqlx::query!(
      "INSERT INTO plantation_pathogenic_occurrences_temperatures (plantation_pathogenic_occurrence_id, date, temperature, quantity)
      VALUES ($1, $2, $3, $4)",
      ocurrence.id,
      NaiveDateTime::parse_from_str(format!("{} 00:00:00", date).as_str(), "%Y-%m-%d %H:%M:%S")
        .unwrap(),
        temperature_celcius.parse::<f64>().unwrap(),
        i32::try_from(temp).unwrap(),
    )
    .execute(&db.pool)
    .await
    .map_err(DataBase::database_error).unwrap();
  }

  for humidity in humidities {
    let humidity_values: Vec<&str> = humidity.0.split("|").collect();
    let date = humidity_values[0];
    let humidity_value = humidity_values[1];

    let temp = humidity.1.len();

    sqlx::query!(
      "INSERT INTO plantation_pathogenic_occurrences_humidities (plantation_pathogenic_occurrence_id, date, humidity, quantity)
      VALUES ($1, $2, $3, $4)",
      ocurrence.id,
      NaiveDateTime::parse_from_str(format!("{} 00:00:00", date).as_str(), "%Y-%m-%d %H:%M:%S")
        .unwrap(),
        humidity_value.parse::<f64>().unwrap(),
        i32::try_from(temp).unwrap(),
    )
    .execute(&db.pool)
    .await
    .map_err(DataBase::database_error).unwrap();
  }

  Ok(())
}
