use crate::AppState;
use axum::Router;

pub mod health;
pub mod users;
pub mod sea_orm_active_enums;

pub fn router() -> Router<AppState> {
  let router_heath: Router<AppState> = health::router();
  let router_users = users::router();

  let routes = Router::new().merge(router_heath).merge(router_users);

  Router::new().nest("/api", routes)
}
