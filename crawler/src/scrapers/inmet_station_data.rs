use chrono::{DateTime, NaiveDateTime, Utc};
use fantoccini::Locator;
use scraper::{Html, Selector};

#[derive(Debug)]
#[allow(dead_code)]
pub(crate) struct InmetStationData {
  pub(crate) date: NaiveDateTime,
  pub(crate) temperature: Vec<f64>,
  pub(crate) humidity: Vec<f64>,
  pub(crate) wind: Vec<f64>,
  pub(crate) pressure: Vec<f64>,
  pub(crate) visibility: Vec<f64>,
  pub(crate) uv_index: f64,
  pub(crate) precipitation: f64,
}

pub(crate) async fn get_station_data(
  client: &fantoccini::Client,
  station: String,
  date: DateTime<Utc>,
) -> Vec<InmetStationData> {
  let url = "https://tempo.inmet.gov.br/TabelaEstacoes/".to_owned();

  client
    .goto(format!("{}{}", url, station).as_str())
    .await
    .unwrap();

  client
    .find(Locator::Css(
      "#root > div.ui.top.attached.header-container.menu > div.left.menu > i",
    ))
    .await
    .unwrap()
    .click()
    .await
    .unwrap();

  let script: &'static str = r#"
      const [date, callback] = arguments;
  
      const input = document.querySelector(`#root > div.pushable.sidebar-content > .menu input[type=date]:first-of-type`)
  
      var nativeInputValueSetter = Object.getOwnPropertyDescriptor(window.HTMLInputElement.prototype, "value").set;
      nativeInputValueSetter.call(input, date);
      
      var ev2 = new Event('input', { bubbles: true});
      input.dispatchEvent(ev2)
    
    "#;

  client
    .execute(script, vec![date.format("%Y-%m-%d").to_string().into()])
    .await
    .unwrap();

  client
    .wait()
    .for_element(Locator::XPath(
      "//*[@id=\"root\"]/div[2]/div[1]/div[2]/button",
    ))
    .await
    .unwrap()
    .click()
    .await
    .unwrap();

  client
    .wait()
    .for_element(Locator::Css(".tabela-body"))
    .await
    .unwrap();

  return parse_stations_climate_data(client.source().await.unwrap()).await;
}

async fn parse_stations_climate_data(html: String) -> Vec<InmetStationData> {
  let mut inmet_stations_data: Vec<InmetStationData> = Vec::new();

  let document = Html::parse_document(&html);
  let row_selector = Selector::parse("tr.tabela-row").unwrap();
  let data_selector = Selector::parse("td.aligned").unwrap();

  for row in document.select(&row_selector) {
    let mut data_values: Vec<String> = Vec::new();

    for data_cell in row.select(&data_selector) {
      let data = data_cell.text().collect::<String>();
      data_values.push(data);
    }

    // Process the data_values vector here
    if data_values.len() == 19 {
      let date = data_values[0].clone();
      let time = data_values[1].clone();
      let temperature_values = &data_values[2..5];
      let humidity_values = &data_values[5..8];
      let wind_values = &data_values[8..11];
      let pressure_values = &data_values[11..14];
      let visibility_values = &data_values[14..17];
      let uv_index = data_values[17].clone();
      let precipitation = data_values[18].clone();

      if temperature_values[0].is_empty() {
        continue;
      }

      inmet_stations_data.push(InmetStationData {
        date: NaiveDateTime::parse_from_str(format!("{} {}", date, time).as_str(), "%d/%m/%Y %H%M")
          .unwrap(),
        temperature: temperature_values
          .iter()
          .map(|value| value.replace(",", ".").parse::<f64>().unwrap_or(0.0))
          .collect::<Vec<f64>>(),
        humidity: humidity_values
          .iter()
          .map(|value| value.replace(",", ".").parse::<f64>().unwrap_or(0.0))
          .collect::<Vec<f64>>(),
        wind: wind_values
          .iter()
          .map(|value| value.replace(",", ".").parse::<f64>().unwrap_or(0.0))
          .collect::<Vec<f64>>(),
        pressure: pressure_values
          .iter()
          .map(|value| value.replace(",", ".").parse::<f64>().unwrap_or(0.0))
          .collect::<Vec<f64>>(),
        visibility: visibility_values
          .iter()
          .map(|value| value.replace(",", ".").parse::<f64>().unwrap_or(0.0))
          .collect::<Vec<f64>>(),
        uv_index: uv_index.replace(",", ".").parse::<f64>().unwrap_or(0.0),
        precipitation: precipitation
          .replace(",", ".")
          .parse::<f64>()
          .unwrap_or(0.0),
      });
    }
  }

  return inmet_stations_data;
}
