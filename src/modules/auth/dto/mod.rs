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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_login_request_serialization() {
    let login_req = LoginRequest {
      email: "test@example.com".to_string(),
      password: "password123".to_string(),
    };

    let json = serde_json::to_string(&login_req).unwrap();
    assert!(json.contains("\"email\":\"test@example.com\""));
    assert!(json.contains("\"password\":\"password123\""));
  }

  #[test]
  fn test_login_request_deserialization() {
    let json = r#"{"email":"user@test.com","password":"secret"}"#;
    let login_req: LoginRequest = serde_json::from_str(json).unwrap();
    assert_eq!(login_req.email, "user@test.com");
    assert_eq!(login_req.password, "secret");
  }

  #[test]
  fn test_register_request_serialization() {
    let register_req = RegisterRequest {
      email: "newuser@example.com".to_string(),
      password: "securepass".to_string(),
      name: "John Doe".to_string(),
    };

    let json = serde_json::to_string(&register_req).unwrap();
    assert!(json.contains("\"email\":\"newuser@example.com\""));
    assert!(json.contains("\"password\":\"securepass\""));
    assert!(json.contains("\"name\":\"John Doe\""));
  }

  #[test]
  fn test_register_request_deserialization() {
    let json = r#"{"email":"jane@test.com","password":"pass123","name":"Jane Smith"}"#;
    let register_req: RegisterRequest = serde_json::from_str(json).unwrap();
    assert_eq!(register_req.email, "jane@test.com");
    assert_eq!(register_req.password, "pass123");
    assert_eq!(register_req.name, "Jane Smith");
  }
}
