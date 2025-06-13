use crate::{
  common::api_error::ApiError,
  modules::health::{dto::Healthy, service},
};
use axum::Json;
use serde_json::Value;

#[utoipa::path(
  get,
  path = "/api/v1/health",
  operation_id = "healthIndex",
  responses(
      (status = 200, description = "Heathy check", body = [Healthy])
  )
)]
pub async fn index() -> Result<Json<Value>, ApiError> {
  let result = service::index().await;
  Ok(Json(result))
}
