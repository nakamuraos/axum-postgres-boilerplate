use axum::{extract::Request, middleware::Next, response::Response};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::common::api_error::ApiError;
use crate::modules::users::entities::Model as User;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
  pub sub: String,
  pub exp: usize,
  pub iat: usize,
  pub email: String,
}

pub async fn auth_guard(req: Request, next: Next) -> Result<Response, ApiError> {
  // Get the authorization header
  let auth_header = req
    .headers()
    .get("authorization")
    .ok_or_else(|| ApiError::Unauthorized("Missing authorization header".to_string()))?
    .to_str()
    .map_err(|_| ApiError::Unauthorized("Invalid authorization header".to_string()))?;

  // Check if it's a Bearer token
  let token = auth_header
    .strip_prefix("Bearer ")
    .ok_or_else(|| ApiError::Unauthorized("Invalid authorization format".to_string()))?;

  // Get JWT secret from environment
  let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "your-secret-key".to_string());

  // Decode and validate the token
  let token_data = decode::<Claims>(
    token,
    &DecodingKey::from_secret(secret.as_bytes()),
    &Validation::default(),
  )
  .map_err(|_| ApiError::Unauthorized("Invalid token".to_string()))?;

  // Check if token is expired
  let now = chrono::Utc::now().timestamp() as usize;
  if token_data.claims.exp < now {
    return Err(ApiError::Unauthorized("Token has expired".to_string()));
  }

  // Get user ID from token
  let user_id = Uuid::parse_str(&token_data.claims.sub)
    .map_err(|_| ApiError::Unauthorized("Invalid user ID in token".to_string()))?;

  // Create a user context from the token claims
  let user = User {
    id: user_id,
    email: token_data.claims.email,
    name: String::new(),     // We don't store name in token
    password: String::new(), // We don't store password in token
    status: crate::modules::users::enums::UserStatus::Active,
    role: crate::modules::users::enums::UserRole::User,
    created_at: None,
    updated_at: None,
  };

  // Add user to request extensions
  let mut req = req;
  req.extensions_mut().insert(user);

  // Continue with the request
  Ok(next.run(req).await)
}
