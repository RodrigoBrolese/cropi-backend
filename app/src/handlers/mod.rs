pub mod health;
mod user;
pub mod login;

use poem::Route;

pub(crate) fn all() -> Route {
  Route::new()
    .nest("/health", health::routes())
    .nest("/users", user::routes())
    .nest("/login", login::routes())
}
