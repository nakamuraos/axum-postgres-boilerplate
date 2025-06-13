pub mod controller;
pub mod dto;
pub mod entities;
pub mod enums;
pub mod service;

use axum::{extract::State, Router};
use axum_extra::routing::Resource;

use crate::app::AppState;
use crate::modules::auth::guards::{admin_guard, auth_guard};

pub fn router(State(state): State<AppState>) -> axum::Router<AppState> {
  let resources = Resource::named("users")
    // Define a route for `GET /users`
    .index(controller::index)
    // `POST /users`
    .create(controller::create)
    // `GET /users/{user_id}`
    .show(controller::show)
    // `PUT or PATCH /users/{user_id}`
    .update(controller::update)
    // `DELETE /users/{user_id}`
    .destroy(controller::destroy);

  Router::new()
    .nest("/v1", Router::new().merge(resources))
    .layer(axum::middleware::from_fn(admin_guard))
    .layer(axum::middleware::from_fn_with_state(state, auth_guard))
}
