use crate::common::api_error::ApiError;
use crate::modules::users::dto::User;
use crate::modules::users::enums::UserRole;
use axum::{extract::Request, middleware::Next, response::Response};
use sea_orm::ActiveEnum;

pub async fn admin_guard(req: Request, next: Next) -> Result<Response, ApiError> {
  // Get the user from request extensions (set by auth_guard)
  let user = req
    .extensions()
    .get::<User>()
    .ok_or_else(|| ApiError::Unauthorized("User not found in request".to_string()))?;

  // Check if user has admin role
  if user.role != UserRole::Admin.to_value() {
    return Err(ApiError::Forbidden("Admin access required".to_string()));
  }

  // Continue with the request
  Ok(next.run(req).await)
}
