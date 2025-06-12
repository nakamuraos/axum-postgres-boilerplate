use crate::modules::users::enums::{UserRole, UserStatus};
use chrono::{DateTime, SecondsFormat, Utc};
use sea_orm::{entity::prelude::*, ActiveValue::Set};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
  #[sea_orm(primary_key, auto_increment = false)]
  pub id: Uuid,
  pub email: String,
  pub name: String,
  pub password: String,
  pub status: UserStatus,
  pub role: UserRole,
  #[sea_orm(column_type = "TimestampWithTimeZone", nullable)]
  pub created_at: Option<DateTime<Utc>>,
  #[sea_orm(column_type = "TimestampWithTimeZone", nullable)]
  pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {
  fn new() -> Self {
    Self {
      id: Set(Uuid::new_v4()),
      status: Set(UserStatus::Inactive),
      role: Set(UserRole::User),
      ..ActiveModelTrait::default()
    }
  }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelatedEntity)]
pub enum RelatedEntity {}

// Custom type for OpenAPI documentation
#[derive(Serialize, Deserialize, ToSchema)]
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
