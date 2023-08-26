use fantoccini::{error::CmdError, Locator};
use scraper::{Html, Selector};
use sqlx::PgPool;
use std::time::Duration;

#[allow(dead_code)]
#[derive(Clone, Debug)]
struct Station {
  city: String,
  uf: String,
  situation: String,
  latitude: String,
  longitude: String,
  altitude: String,
  installation_date: String,
  code: String,
}

#[derive(Clone, serde::Serialize, Debug)]
struct StationDB {
  id: i64,
  inmet_code: Option<String>,
}

async fn parse_stations(html: String) -> Vec<Station> {
  let document = Html::parse_document(&html);

  let row_selector = Selector::parse("tr").unwrap();
  let cell_selector = Selector::parse("td").unwrap();

  let mut stations = Vec::new();

  for row in document.select(&row_selector) {
    let mut cells = row.select(&cell_selector);

    if cells.clone().count() == 0 {
      continue;
    }

    let city = cells.next().unwrap().text().collect::<String>();
    let uf = cells.next().unwrap().text().collect::<String>();
    let situation = cells.next().unwrap().text().collect::<String>();
    let latitude = cells.next().unwrap().text().collect::<String>();
    let longitude = cells.next().unwrap().text().collect::<String>();
    let altitude = cells.next().unwrap().text().collect::<String>();
    let installation_date = cells.next().unwrap().text().collect::<String>();
    let code = cells.next().unwrap().text().collect::<String>();

    let station = Station {
      city,
      uf,
      situation,
      latitude,
      longitude,
      altitude,
      installation_date,
      code,
    };

    stations.push(station);
  }

  stations
}

pub(crate) async fn handler(client: &fantoccini::Client) -> Result<(), CmdError> {
  let pool = PgPool::connect(std::env::var("DATABASE_URL").unwrap().as_str())
    .await
    .expect("Error connecting to database");

  client
    .goto("https://portal.inmet.gov.br/paginas/catalogoaut")
    .await
    .unwrap();

  client
    .wait()
    .at_most(Duration::from_secs(10))
    .for_element(Locator::Css("#tb"))
    .await
    .unwrap();

  let html = client.source().await.unwrap();

  let stations = parse_stations(html).await;

  let stations_code = stations
    .clone()
    .into_iter()
    .map(|s| s.code)
    .collect::<Vec<String>>();

  let stations_db = sqlx::query_as!(
    StationDB,
    "SELECT id,
        inmet_code
    FROM stations
    WHERE inmet_code = ANY($1)",
    &stations_code[..]
  )
  .fetch_all(&pool)
  .await
  .expect("error on find stations on the DB");

  for station in stations {
    let station_db = stations_db
      .to_owned()
      .into_iter()
      .find(|s| s.to_owned().inmet_code.unwrap_or("".to_string()) == station.code);

    let point = format!(
      "POINT ({} {})",
      station.latitude.to_string().replace(",", "."),
      station.longitude.to_string().replace(",", ".")
    );

    let status = station.situation == "Operante";

    if station_db.is_some() {
      sqlx::query!(
        "UPDATE stations SET city = $2, status = $3, location = ST_PointFromText($4)::point WHERE id = $1;",
        station_db.unwrap().id,
        station.city,
        status,
        point
      )
      .execute(&pool)
      .await
      .unwrap();
      continue;
    }

    sqlx::query!(
      "INSERT INTO stations (city, uf, location, status, inmet_code)
                VALUES ($1, $2, ST_PointFromText($3)::point, $4, $5);",
      station.city,
      station.uf,
      point,
      status,
      station.code
    )
    .execute(&pool)
    .await
    .unwrap();
  }

  Ok(())
}
