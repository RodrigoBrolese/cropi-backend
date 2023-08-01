use poem::{get, handler, web::Json, Route};

#[handler]
fn health() -> Json<serde_json::Value> {
  Json(serde_json::json! ({
    "message": "ok",
  }))
}

pub fn routes() -> Route {
  Route::new().at("/", get(health).post(health))
}
