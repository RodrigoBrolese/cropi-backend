use chrono::Utc;
use fantoccini::{error::CmdError, Locator};
use scraper::{Html, Selector};
use std::{thread, time::Duration};

async fn get_station_data(client: &fantoccini::Client, station: String) {
  let url = "https://tempo.inmet.gov.br/TabelaEstacoes/".to_owned();

  let date = Utc::now() - chrono::Duration::days(1);
  // first, go to the Wikipedia page for Foobar

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

  thread::sleep(Duration::from_secs(3));

  client
    .find(Locator::XPath(
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

  parse_stations_climate_data(client.source().await.unwrap()).await;
}

async fn parse_stations_climate_data(html: String) {
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

      println!("Date: {}", date);
      println!("Time: {}", time);
      println!("Temperature Values: {:?}", temperature_values);
      println!("Humidity Values: {:?}", humidity_values);
      println!("Wind Values: {:?}", wind_values);
      println!("Pressure Values: {:?}", pressure_values);
      println!("Visibility Values: {:?}", visibility_values);
      println!("UV Index: {}", uv_index);
      println!("Precipitation: {}", precipitation);
      println!();
    }
  }
}

pub(crate) async fn handler(client: &fantoccini::Client) -> Result<(), CmdError> {
  get_station_data(&client, "A814".to_string()).await;

  Ok(())
}
