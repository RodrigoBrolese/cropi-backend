use poem::{http::StatusCode, Endpoint, Error, Request, Result};

pub(crate) async fn handle<E: Endpoint>(next: E, req: Request) -> Result<<E as Endpoint>::Output> {
  let content_type = req.header("content-type").unwrap_or("");

  if !content_type.contains("application/json") {
    return Err(Error::from_string(
      "Content-Type must be JSON".to_string(),
      StatusCode::BAD_REQUEST,
    ));
  }

  return next.call(req).await;
}
