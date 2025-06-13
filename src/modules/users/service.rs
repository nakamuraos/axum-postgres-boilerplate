use bcrypt::{hash, DEFAULT_COST};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use uuid::Uuid;

use crate::common::api_error::ApiError;
use crate::modules::users::dto::UserDto;
use crate::modules::users::entities::{self, Entity as UserEntity};
use crate::modules::users::enums::UserStatus;

pub async fn index(db: &DatabaseConnection) -> Result<serde_json::Value, ApiError> {
  let users = UserEntity::find().all(db).await?;
  let responses: Vec<UserDto> = users.into_iter().map(UserDto::from).collect();
  Ok(serde_json::json!(responses))
}

pub async fn create(
  db: &DatabaseConnection,
  email: String,
  password: String,
  name: String,
) -> Result<serde_json::Value, ApiError> {
  // Hash password
  let password_hash = hash(password.as_bytes(), DEFAULT_COST)
    .map_err(|e| ApiError::InternalError(anyhow::anyhow!("Failed to hash password: {}", e)))?;

  let user = entities::ActiveModel {
    id: Set(Uuid::new_v4()),
    email: Set(email),
    password: Set(password_hash),
    name: Set(name),
    status: Set(UserStatus::Active),
    ..Default::default()
  };

  let user = user.insert(db).await.map_err(|e| {
    if e.to_string().contains("duplicate key") {
      ApiError::InvalidRequest("Email already exists".to_string())
    } else {
      ApiError::InternalError(anyhow::anyhow!(e))
    }
  })?;

  let response = UserDto::from(user);
  Ok(serde_json::json!(response))
}

pub async fn show(db: &DatabaseConnection, id: Uuid) -> Result<serde_json::Value, ApiError> {
  let user = UserEntity::find()
    .filter(entities::Column::Id.eq(id))
    .one(db)
    .await?
    .ok_or_else(|| ApiError::NotFound("User not found".to_string()))?;

  let response = UserDto::from(user);
  Ok(serde_json::json!(response))
}

pub async fn update(
  db: &DatabaseConnection,
  id: Uuid,
  name: String,
) -> Result<serde_json::Value, ApiError> {
  let user = UserEntity::find()
    .filter(entities::Column::Id.eq(id))
    .one(db)
    .await?
    .ok_or_else(|| ApiError::NotFound("User not found".to_string()))?;

  let mut user: entities::ActiveModel = user.into();
  user.name = Set(name);

  let user = user.update(db).await?;
  let response = UserDto::from(user);
  Ok(serde_json::json!(response))
}

pub async fn destroy(db: &DatabaseConnection, id: Uuid) -> Result<(), ApiError> {
  let user = UserEntity::find()
    .filter(entities::Column::Id.eq(id))
    .one(db)
    .await?
    .ok_or_else(|| ApiError::NotFound("User not found".to_string()))?;

  let user: entities::ActiveModel = user.into();
  user.delete(db).await?;
  Ok(())
}
