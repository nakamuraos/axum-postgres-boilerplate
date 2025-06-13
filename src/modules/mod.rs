pub mod auth;
pub mod health;
pub mod users;

use crate::app::AppState;
use axum::{extract::State, Router};

pub fn router(State(state): State<AppState>) -> Router<AppState> {
  let router_health: Router<AppState> = health::router();
  let router_auth: Router<AppState> = auth::router();
  let router_users: Router<AppState> = users::router(axum::extract::State(state));

  let routes: Router<AppState> = Router::new()
    .merge(router_health)
    .merge(router_auth)
    .merge(router_users);

  Router::new().nest("/api", routes)
}
