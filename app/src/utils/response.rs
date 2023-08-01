use poem::{http::StatusCode, web::Json, Response};
use serde::Serialize;

#[derive(Serialize)]
pub(crate) struct JsonError {
  field: String,
  message: String,
}

impl JsonError {
  pub fn new(field: String, message: String) -> JsonError {
    JsonError { field, message }
  }
}

pub(crate) fn json(body: serde_json::Value, status: StatusCode) -> Response {
  let json_body: Json<serde_json::Value> = Json(serde_json::from_value(body).unwrap());

  Response::builder()
    .content_type("aplication/json")
    .status(status)
    .body(json_body.to_string())
}

pub(crate) fn json_ok(body: serde_json::Value) -> Response {
  json(body, StatusCode::OK)
}

pub(crate) fn garde_error_to_json(e: garde::Errors) -> serde_json::Value {
  let mut result = Vec::new();

  let errors = e.flatten().into_iter();
  for error in errors {
    result.push(JsonError {
      field: error.0.replace("value.", ""),
      message: error.1.to_string(),
    });
  }
  serde_json::json!({
      "errors": result
  })
}
