use crate::modules::users::entities::UserResponse;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct User {
  pub id: String,
  pub name: String,
  pub status: String,
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
      status: response.status,
    }
  }
}
