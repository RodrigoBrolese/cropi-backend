pub mod handlers;

use dotenv::dotenv;
use poem::{
  error::NotFoundError,
  http::StatusCode,
  listener::TcpListener,
  middleware::Tracing,
  EndpointExt,
  Response,
  Route,
  Server,
};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
  dotenv().ok();

  if std::env::var_os("RUST_LOG").is_none() {
    std::env::set_var("RUST_LOG", "poem=debug");
  }
  tracing_subscriber::fmt::init();

  let app = Route::new()
    .nest("/", handlers::all())
    .with(Tracing)
    .catch_error(|_: NotFoundError| async move {
      Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body("{message: \"Not Found\"}")
    });

  Server::new(TcpListener::bind("127.0.0.1:3000"))
    .name("cropi")
    .run(app)
    .await
}
