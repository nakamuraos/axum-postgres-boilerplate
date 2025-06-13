use crate::modules::users::entities::UserResponse;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Serialize, Deserialize, ToSchema)]
pub struct User {
  pub id: String,
  pub name: String,
  pub email: String,
  pub status: String,
  pub role: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct UserCreate {
  pub email: String,
  pub password: String,
  pub name: String,
}

impl From<UserResponse> for User {
  fn from(response: UserResponse) -> Self {
    Self {
      id: response.id,
      name: response.name,
      email: response.email,
      status: response.status,
      role: response.role,
    }
  }
}
