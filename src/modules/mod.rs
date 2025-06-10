pub mod health;
pub mod sea_orm_active_enums;
pub mod users;

use crate::AppState;
use axum::Router;

pub fn router() -> Router<AppState> {
  let router_health: Router<AppState> = health::router();
  let router_users: Router<AppState> = users::router();

  let routes: Router<AppState> = Router::new().merge(router_health).merge(router_users);

  Router::new().nest("/api", routes)
}
