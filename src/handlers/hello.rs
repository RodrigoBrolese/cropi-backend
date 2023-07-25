use poem::{get, handler, web::Json, Route};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct CreateSomething {
  name: String,
}

#[handler]
fn hello() -> Json<serde_json::Value> {
  Json(serde_json::json! ({
    "code": 0,
    "message": "dsadas",
  }))
}

#[handler]
fn hello_post(req: Json<CreateSomething>) -> Json<serde_json::Value> {
  Json(serde_json::json! ({
      "code": 0,
      "message": req.name,
  }))
}

pub fn routes() -> Route {
  Route::new().at("/", get(hello).post(hello_post))
}
