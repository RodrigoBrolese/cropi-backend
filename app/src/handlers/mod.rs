pub mod health;
pub mod login;
pub mod plantations;
pub mod user;

use crate::middleware::{auth, ensure_json};
use poem::{endpoint::StaticFilesEndpoint, EndpointExt, Route};

pub(crate) fn all() -> Route {
  Route::new()
    .nest("/health", health::routes())
    .nest("/user", user::routes().around(ensure_json::handle))
    .nest("/login", login::routes().around(ensure_json::handle))
    .nest("/plantations", plantations::routes().around(auth::handle))
    .nest(
      "/images/ocurrences",
      StaticFilesEndpoint::new("app/images/ocurrences/").show_files_listing(),
    )
}
