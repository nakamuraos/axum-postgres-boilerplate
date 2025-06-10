pub mod dto;
pub mod service;

use crate::{common::api_error::ApiError, modules::health::dto::Healthy, AppState};
use axum::{Json, Router};
use axum_extra::routing::Resource;
use serde_json::Value;

pub fn router() -> axum::Router<AppState> {
  let resources_v1 = Resource::named("health").index(index);

  Router::new().nest("/v1", Router::new().merge(resources_v1))
}

#[utoipa::path(
  get,
  path = "/api/v1/health",
  operation_id = "healthIndex",
  responses(
      (status = 200, description = "Heathy check", body = [Healthy])
  )
)]
async fn index() -> Result<Json<Value>, ApiError> {
  let result = service::index().await;
  Ok(Json(result))
}
