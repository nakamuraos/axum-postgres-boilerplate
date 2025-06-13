pub mod dto;
pub mod guards;
pub mod service;

use axum::{extract::State, Json, Router};
use serde_json::Value;

use crate::common::api_error::ApiError;
use crate::modules::auth::dto::{AuthResponse, LoginRequest, RegisterRequest};
use crate::app::AppState;

pub fn router() -> Router<AppState> {
  Router::new()
    .route("/v1/auth/register", axum::routing::post(register))
    .route("/v1/auth/login", axum::routing::post(login))
}

#[utoipa::path(
  post,
  path = "/api/v1/auth/register",
  operation_id = "authRegister",
  request_body = RegisterRequest,
  responses(
    (status = 200, description = "Register successful", body = AuthResponse),
    (status = 409, description = "Email already exists"),
    (status = 500, description = "Internal server error")
  )
)]
pub async fn register(
  State(state): State<AppState>,
  Json(req): Json<RegisterRequest>,
) -> Result<Json<Value>, ApiError> {
  let result = service::register(&state.db.conn, req).await?;
  Ok(Json(result))
}

#[utoipa::path(
  post,
  path = "/api/v1/auth/login",
  operation_id = "authLogin",
  request_body = LoginRequest,
  responses(
    (status = 200, description = "Login successful", body = AuthResponse),
    (status = 401, description = "Invalid credentials"),
    (status = 500, description = "Internal server error")
  )
)]
pub async fn login(
  State(state): State<AppState>,
  Json(req): Json<LoginRequest>,
) -> Result<Json<Value>, ApiError> {
  let result = service::login(&state.db.conn, req).await?;
  Ok(Json(result))
}
