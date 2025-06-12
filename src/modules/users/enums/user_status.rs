use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "user_status")]
pub enum UserStatus {
  #[sea_orm(string_value = "Active")]
  Active,
  #[sea_orm(string_value = "Inactive")]
  Inactive,
  #[sea_orm(string_value = "Banned")]
  Banned,
}
