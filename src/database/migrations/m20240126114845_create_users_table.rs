use crate::modules::users::enums::{UserRole, UserStatus};
use sea_orm::{ActiveEnum, DbBackend, Schema, Statement};
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    let schema = Schema::new(DbBackend::Postgres);

    // Check if enum type exists before creating it
    let db = manager.get_connection();
    let enum_name = UserStatus::name().to_string();
    let check_type = format!(
      "SELECT EXISTS (
        SELECT 1 FROM pg_type 
        WHERE typname = '{}'
      )",
      enum_name
    );
    let type_exists: bool = db
      .query_one(Statement::from_string(DbBackend::Postgres, check_type))
      .await?
      .map(|row| row.try_get::<bool>("", "exists").unwrap_or(false))
      .unwrap_or(false);

    if !type_exists {
      // Create the enum type for Status
      manager
        .create_type(schema.create_enum_from_active_enum::<UserStatus>())
        .await?;
    }

    // Check if enum type exists before creating it
    let db = manager.get_connection();
    let enum_name = UserRole::name().to_string();
    let check_type = format!(
      "SELECT EXISTS (
        SELECT 1 FROM pg_type 
        WHERE typname = '{}'
      )",
      enum_name
    );
    let type_exists: bool = db
      .query_one(Statement::from_string(DbBackend::Postgres, check_type))
      .await?
      .map(|row| row.try_get::<bool>("", "exists").unwrap_or(false))
      .unwrap_or(false);

    if !type_exists {
      // Create the enum type for Role
      manager
        .create_type(schema.create_enum_from_active_enum::<UserRole>())
        .await?;
    }

    // Create the users table
    manager
      .create_table(
        Table::create()
          .table(Users::Table)
          .if_not_exists()
          .col(ColumnDef::new(Users::Id).uuid().not_null().primary_key())
          .col(ColumnDef::new(Users::Email).string().not_null())
          .col(ColumnDef::new(Users::Password).string().not_null())
          .col(ColumnDef::new(Users::Name).string().not_null())
          .col(
            ColumnDef::new(Users::Status)
              .custom(UserStatus::name())
              .not_null()
              .default(Expr::value("Inactive")),
          )
          .col(
            ColumnDef::new(Users::Role)
              .custom(UserRole::name())
              .not_null()
              .default(Expr::value("User")),
          )
          .col(
            ColumnDef::new(Users::CreatedAt)
              .timestamp_with_time_zone()
              .not_null()
              .default(Expr::current_timestamp()),
          )
          .col(
            ColumnDef::new(Users::UpdatedAt)
              .timestamp_with_time_zone()
              .not_null()
              .default(Expr::current_timestamp()),
          )
          .to_owned(),
      )
      .await
  }

  async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    manager
      .drop_table(Table::drop().table(Users::Table).to_owned())
      .await
  }
}

#[derive(Iden)]
enum Users {
  Table,
  Id,
  Email,
  Password,
  Name,
  Status,
  Role,
  CreatedAt,
  UpdatedAt,
}
