use chrono::SecondsFormat;
use sea_orm::ActiveEnum;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::modules::users::entities::Model;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct UserCreate {
  pub email: String,
  pub password: String,
  pub name: String,
}

// Custom type for OpenAPI documentation
#[derive(Debug, Default, Clone, Serialize, Deserialize, ToSchema)]
pub struct UserResponse {
  pub id: String,
  pub email: String,
  pub name: String,
  pub status: String,
  pub role: String,
  #[schema(format = "date-time")]
  pub created_at: Option<String>,
  #[schema(format = "date-time")]
  pub updated_at: Option<String>,
}

impl From<Model> for UserResponse {
  fn from(model: Model) -> Self {
    Self {
      id: model.id.to_string(),
      email: model.email,
      name: model.name,
      status: model.status.into_value(),
      role: model.role.into_value(),
      created_at: model
        .created_at
        .map(|dt| dt.to_rfc3339_opts(SecondsFormat::Millis, true)),
      updated_at: model
        .updated_at
        .map(|dt| dt.to_rfc3339_opts(SecondsFormat::Millis, true)),
    }
  }
}
