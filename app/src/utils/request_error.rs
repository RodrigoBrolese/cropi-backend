use poem::{http::StatusCode, Error, IntoResponse};

pub(crate) async fn catch_all_errors(err: Error) -> impl IntoResponse {
  super::response::json(
    serde_json::json!({
        "error": err.to_string()
    }),
    err.status(),
  )
}

pub(crate) fn catch_not_found_error() -> impl IntoResponse {
  super::response::json(
    serde_json::json!({
      "error": "Not Found"
    }),
    StatusCode::NOT_FOUND,
  )
}

pub(crate) fn catch_panic() -> impl IntoResponse {
  super::response::json(
    serde_json::json!({
      "error": "Internal server error"
    }),
    StatusCode::INTERNAL_SERVER_ERROR,
  )
}
