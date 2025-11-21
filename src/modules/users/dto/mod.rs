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
pub struct UserDto {
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

impl From<Model> for UserDto {
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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_user_create_serialization() {
    let user = UserCreate {
      email: "test@example.com".to_string(),
      password: "secure123".to_string(),
      name: "Test User".to_string(),
    };

    let json = serde_json::to_string(&user).unwrap();
    assert!(json.contains("\"email\":\"test@example.com\""));
    assert!(json.contains("\"password\":\"secure123\""));
    assert!(json.contains("\"name\":\"Test User\""));
  }

  #[test]
  fn test_user_create_deserialization() {
    let json = r#"{"email":"john@test.com","password":"pass456","name":"John Doe"}"#;
    let user: UserCreate = serde_json::from_str(json).unwrap();
    assert_eq!(user.email, "john@test.com");
    assert_eq!(user.password, "pass456");
    assert_eq!(user.name, "John Doe");
  }

  #[test]
  fn test_user_dto_default() {
    let dto = UserDto::default();
    assert_eq!(dto.id, "");
    assert_eq!(dto.email, "");
    assert_eq!(dto.name, "");
    assert_eq!(dto.status, "");
    assert_eq!(dto.role, "");
    assert!(dto.created_at.is_none());
    assert!(dto.updated_at.is_none());
  }

  #[test]
  fn test_user_dto_serialization() {
    let dto = UserDto {
      id: "123e4567-e89b-12d3-a456-426614174000".to_string(),
      email: "user@test.com".to_string(),
      name: "Test User".to_string(),
      status: "Active".to_string(),
      role: "User".to_string(),
      created_at: Some("2024-01-01T00:00:00.000Z".to_string()),
      updated_at: Some("2024-01-02T00:00:00.000Z".to_string()),
    };

    let json = serde_json::to_string(&dto).unwrap();
    assert!(json.contains("\"id\":\"123e4567-e89b-12d3-a456-426614174000\""));
    assert!(json.contains("\"email\":\"user@test.com\""));
    assert!(json.contains("\"name\":\"Test User\""));
    assert!(json.contains("\"status\":\"Active\""));
    assert!(json.contains("\"role\":\"User\""));
  }

  #[test]
  fn test_user_dto_deserialization() {
    let json = r#"{
      "id":"550e8400-e29b-41d4-a716-446655440000",
      "email":"jane@example.com",
      "name":"Jane Smith",
      "status":"Inactive",
      "role":"Admin",
      "created_at":"2024-01-01T12:00:00.000Z",
      "updated_at":"2024-01-01T12:00:00.000Z"
    }"#;
    let dto: UserDto = serde_json::from_str(json).unwrap();
    assert_eq!(dto.id, "550e8400-e29b-41d4-a716-446655440000");
    assert_eq!(dto.email, "jane@example.com");
    assert_eq!(dto.name, "Jane Smith");
    assert_eq!(dto.status, "Inactive");
    assert_eq!(dto.role, "Admin");
    assert!(dto.created_at.is_some());
    assert!(dto.updated_at.is_some());
  }
}
