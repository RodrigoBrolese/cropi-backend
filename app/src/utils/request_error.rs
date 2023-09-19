use poem::{http::StatusCode, Error, IntoResponse};

pub(crate) async fn catch_all_errors(mut err: Error) -> impl IntoResponse {
  let mut error = err.to_string();

  if err.is::<poem::error::ParseJsonError>() {
    error = err.to_string();
    err.set_error_message("Check your content-type and JSON syntax");
  }

  super::response::json(
    serde_json::json!({
        "message": err.to_string(),
        "error": error,
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
