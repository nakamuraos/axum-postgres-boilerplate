use crate::modules::users::enums::{UserRole, UserStatus};
use chrono::{DateTime, Utc};
use sea_orm::{entity::prelude::*, ActiveValue::Set};
use serde::{Deserialize, Serialize};

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
