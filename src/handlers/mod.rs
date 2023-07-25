pub mod hello;

use poem::Route;

pub(crate) fn all() -> Route {
  Route::new().nest("/hello", hello::routes())
}
