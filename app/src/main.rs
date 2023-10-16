pub mod handlers;
pub mod jobs;
pub mod middleware;
pub mod models;
pub mod utils;

use dotenv::dotenv;
use poem::{
  error::NotFoundError,
  listener::TcpListener,
  middleware::{CatchPanic, Cors, Tracing},
  EndpointExt,
  Route,
  Server,
};
use utils::database::DataBase;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
  dotenv().ok();

  if std::env::var_os("RUST_LOG").is_none() {
    std::env::set_var("RUST_LOG", "poem=debug");
  }
  tracing_subscriber::fmt::init();

  let db: DataBase = DataBase::new().await;

  let app = Route::new()
    .nest("/", handlers::all())
    .with(Tracing)
    .with(Cors::new())
    .catch_error(|_: NotFoundError| async move { utils::request_error::catch_not_found_error() })
    .catch_all_error(utils::request_error::catch_all_errors)
    .with(CatchPanic::new().with_handler(|_| utils::request_error::catch_panic()))
    .data(db);

  Server::new(TcpListener::bind(std::env::var("APP_URL").unwrap()))
    .name("cropi")
    .run(app)
    .await
}
