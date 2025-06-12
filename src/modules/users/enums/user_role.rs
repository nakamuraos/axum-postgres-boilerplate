use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "user_role")]
pub enum UserRole {
  #[sea_orm(string_value = "Admin")]
  Admin,
  #[sea_orm(string_value = "User")]
  User,
}

impl Default for UserRole {
  fn default() -> Self {
    Self::User
  }
}
