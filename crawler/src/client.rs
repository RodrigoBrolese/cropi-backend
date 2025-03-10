use fantoccini::{error::NewSessionError, Client, ClientBuilder};
use ua_generator::ua::spoof_ua;

pub(crate) async fn make() -> Result<Client, NewSessionError> {
  let mut capabiliteies = serde_json::map::Map::new();

  capabiliteies.insert(
    "goog:chromeOptions".into(),
    serde_json::json!({
      "args": vec![
        "--headless",
        "--no-sandbox",
        "--disable-gpu",
        "--disable-remote-fonts",
        "--window-size=1920,1080",
        format!("--user-agent={}", spoof_ua()).as_str()
      ]
    }),
  );

  return ClientBuilder::native()
    .capabilities(capabiliteies)
    .connect("http://localhost:4444")
    .await;
}
