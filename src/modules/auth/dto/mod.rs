use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::modules::users::dto::UserDto;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct LoginRequest {
  pub email: String,
  pub password: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct RegisterRequest {
  pub email: String,
  pub password: String,
  pub name: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct AuthResponse {
  pub token: String,
  pub user: UserDto,
}
