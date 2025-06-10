use async_graphql::dynamic::*;
use sea_orm::DatabaseConnection;
use seaography::{async_graphql, lazy_static, Builder, BuilderContext};
use server::modules::users::entities;

lazy_static::lazy_static! { static ref CONTEXT : BuilderContext = BuilderContext :: default () ; }

pub fn schema(
  database: DatabaseConnection,
  depth: Option<usize>,
  complexity: Option<usize>,
) -> Result<Schema, SchemaError> {
  // Create a new schema builder with the provided database connection
  let mut builder = Builder::new(&CONTEXT, database.clone());

  // Register the entities
  seaography::register_entities!(builder, [entities]);

  // Register the active enums
  builder.register_enumeration::<server::modules::sea_orm_active_enums::MpaaRating>();
  builder.register_enumeration::<server::modules::users::enums::UserStatus>();

  // Register the custom scalars
  builder
    .set_depth_limit(depth)
    .set_complexity_limit(complexity)
    .schema_builder()
    .data(database)
    .finish()
}
