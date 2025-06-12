use anyhow::anyhow;
use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{encode, EncodingKey, Header};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

use crate::common::api_error::ApiError;
use crate::modules::auth::dto::{AuthResponse, LoginRequest, RegisterRequest, UserResponse};
use crate::modules::users::entities::{self as User};
use crate::modules::users::enums::{UserRole, UserStatus};

#[derive(Serialize, Deserialize, Default)]
pub struct Claims {
  pub sub: Option<String>,
  pub exp: Option<usize>,
  pub iat: usize,
  pub email: String,
  pub status: UserStatus,
  pub role: UserRole,
}

pub async fn register(conn: &DatabaseConnection, req: RegisterRequest) -> Result<Value, ApiError> {
  // Hash password
  let password_hash = hash(req.password.as_bytes(), DEFAULT_COST)
    .map_err(|e| ApiError::InternalError(anyhow!("Failed to hash password: {}", e)))?;

  // Create user
  let user = User::ActiveModel {
    id: sea_orm::ActiveValue::Set(Uuid::new_v4()),
    email: sea_orm::ActiveValue::Set(req.email),
    password: sea_orm::ActiveValue::Set(password_hash),
    name: sea_orm::ActiveValue::Set(req.name),
    ..Default::default()
  };

  let user = user.insert(conn).await.map_err(|e| {
    if e.to_string().contains("duplicate key") {
      ApiError::InvalidRequest("Email already exists".to_string())
    } else {
      ApiError::InternalError(anyhow!(e))
    }
  })?;

  // Generate JWT token
  let token = generate_token(&user)?;

  let response = AuthResponse {
    token,
    user: UserResponse {
      id: user.id.to_string(),
      email: user.email,
      name: user.name,
    },
  };

  Ok(serde_json::to_value(response).map_err(|e| ApiError::InternalError(anyhow!(e)))?)
}

pub async fn login(conn: &DatabaseConnection, req: LoginRequest) -> Result<Value, ApiError> {
  // Find user by email
  let user = User::Entity::find()
    .filter(User::Column::Email.eq(req.email))
    .one(conn)
    .await?
    .ok_or_else(|| ApiError::InvalidRequest("Invalid credentials".to_string()))?;

  // Verify password
  if !verify(req.password, &user.password)
    .map_err(|e| ApiError::InternalError(anyhow!("Failed to verify password: {}", e)))?
  {
    return Err(ApiError::InvalidRequest("Invalid credentials".to_string()));
  }

  // Generate JWT token
  let token = generate_token(&user)?;

  let response = AuthResponse {
    token,
    user: UserResponse {
      id: user.id.to_string(),
      email: user.email,
      name: user.name,
    },
  };

  Ok(serde_json::to_value(response).map_err(|e| ApiError::InternalError(anyhow!(e)))?)
}

fn generate_token(user: &User::Model) -> Result<String, ApiError> {
  let secret = std::env::var("JWT_SECRET")
    .unwrap_or_else(|_| "a-string-secret-at-least-256-bits-long".to_string());
  let expiration = chrono::Utc::now()
    .checked_add_signed(chrono::Duration::days(7))
    .expect("valid timestamp")
    .timestamp();

  let claims = Claims {
    sub: Some(user.id.to_string()),
    exp: Some(expiration as usize),
    email: user.email.clone(),
    status: user.status.clone(),
    role: user.role.clone(),
    ..Default::default()
  };

  encode(
    &Header::default(),
    &claims,
    &EncodingKey::from_secret(secret.as_bytes()),
  )
  .map_err(|e| ApiError::InternalError(anyhow!("Failed to generate token: {}", e)))
}
