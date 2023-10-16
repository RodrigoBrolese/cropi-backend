use clap::Parser;
use dotenv::dotenv;
use handlers::{
  inmet_stations,
  inmet_temperature_data,
  ocurrence_inmet_ocorrence_data,
  ocurrence_inmet_probability,
};

pub mod client;
mod handlers;
mod scrapers;
pub mod utils;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
  #[arg(short, long)]
  script: String,
  #[arg(short, long)]
  ocurrence_id: Option<String>,
  #[arg(short, long)]
  pathogenic_id: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), fantoccini::error::CmdError> {
  dotenv().ok();

  let args = Args::parse();

  let client = client::make().await.expect("error on client build");

  match args.script.as_str() {
    "ocurrence-immet-climate-data" => {
      ocurrence_inmet_ocorrence_data::handler(&client, args.ocurrence_id.unwrap())
        .await
        .unwrap();
    }
    "inmet-stations" => inmet_stations::handler(&client).await.unwrap(),
    "ocurrence-probability" => {
      ocurrence_inmet_probability::handler(&client, args.pathogenic_id.unwrap())
        .await
        .unwrap();
    }
    "inmet-temperature-data" => {
      inmet_temperature_data::handler(&client).await.unwrap();
    }
    _ => panic!("script not found"),
  }

  client.close().await
}
