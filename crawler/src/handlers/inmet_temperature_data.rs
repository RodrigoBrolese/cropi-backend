use crate::scrapers::inmet_station_data::get_station_data;
use chrono::Utc;
use fantoccini::error::CmdError;
use std::collections::HashMap;

pub(crate) async fn handler(client: &fantoccini::Client) -> Result<(), CmdError> {
  let stations_data = get_station_data(
    &client,
    "A814".to_string(),
    Utc::now() - chrono::Duration::days(7),
  )
  .await;

  let mut temperatures: HashMap<String, Vec<String>> = HashMap::new();
  let mut humidities: HashMap<String, Vec<String>> = HashMap::new();

  for data in stations_data {
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

  //order temperatures by date
  let mut temperatures: Vec<(&String, &Vec<String>)> = temperatures.iter().collect();
  temperatures.sort_by(|a, b| a.0.cmp(b.0));

  //order humidities by date
  let mut humidities: Vec<(&String, &Vec<String>)> = humidities.iter().collect();
  humidities.sort_by(|a, b| a.0.cmp(b.0));

  for temperature in temperatures {
    let temperature_values: Vec<&str> = temperature.0.split("|").collect();
    let date = temperature_values[0];
    let temperature_celcius = temperature_values[1];

    let qtd = temperature.1.len();

    if temperature_celcius.parse::<f64>().unwrap() >= 17.0
      && temperature_celcius.parse::<f64>().unwrap() <= 25.0
    {
      println!(
        "dia: {} - temperatura: {} - qtd:{}",
        date, temperature_celcius, qtd
      );
    }
  }

  for humidity in humidities {
    let humidity_values: Vec<&str> = humidity.0.split("|").collect();
    let date = humidity_values[0];
    let humidity_value = humidity_values[1];

    let qtd = humidity.1.len();

    if humidity_value.parse::<f64>().unwrap() >= 90.0 {
      println!("dia: {} - umidade: {} - qtd:{}", date, humidity_value, qtd);
    }
  }

  Ok(())
}
