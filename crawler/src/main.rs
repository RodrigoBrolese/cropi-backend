use clap::Parser;
use dotenv::dotenv;
use handlers::{climate_data, inmet_stations};

pub mod client;
mod handlers;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
  #[arg(short, long)]
  script: String,
}

#[tokio::main]
async fn main() -> Result<(), fantoccini::error::CmdError> {
  dotenv().ok();

  let args = Args::parse();

  let client = client::make().await.expect("error on client build");

  match args.script.as_str() {
    "climate-data" => {
      climate_data::handler(&client).await.unwrap();
    }
    "inmet-stations" => inmet_stations::handler(&client).await.unwrap(),
    _ => panic!("script not found"),
  }

  client.close().await
}
