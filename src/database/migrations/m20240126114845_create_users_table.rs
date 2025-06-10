use crate::modules::users::enums::UserStatus;
use sea_orm::{ActiveEnum, DbBackend, Schema};
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
  async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
    let schema = Schema::new(DbBackend::Postgres);

    // Create the enum type for Status
    // CREATE TYPE "user_status" AS ENUM ('Active', 'Inactive', 'Banned')
    manager
      .create_type(schema.create_enum_from_active_enum::<UserStatus>())
      .await?;

    // Create the users table
    // CREATE TABLE "users" (
    //   "id" UUID NOT NULL PRIMARY KEY,
    //   "name" TEXT NOT NULL,
    //   "status" "user_status" NOT NULL,
    //   "created_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    //   "updated_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
    // );
    manager
      .create_table(
        Table::create()
          .table(Users::Table)
          .if_not_exists()
          .col(ColumnDef::new(Users::Id).uuid().not_null().primary_key())
          .col(ColumnDef::new(Users::Name).string().not_null())
          .col(
            ColumnDef::new(Users::Status)
              .custom(UserStatus::name())
              .not_null(),
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
  Name,
  Status,
  CreatedAt,
  UpdatedAt,
}
