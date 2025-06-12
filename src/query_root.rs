use crate::modules::auth::guards::graphql_guards;
use crate::modules::users::{self, entities as usersEntities};
use async_graphql::dynamic::*;
use sea_orm::DatabaseConnection;
use seaography::{async_graphql, lazy_static, Builder, BuilderContext};

lazy_static::lazy_static! {
  static ref CONTEXT: BuilderContext = {
    let context = BuilderContext::default();
    let guards = graphql_guards::setup_guards();

    BuilderContext {
      guards,
      ..context
    }
  };
}

pub fn schema(
  database: DatabaseConnection,
  depth: Option<usize>,
  complexity: Option<usize>,
) -> Result<Schema, SchemaError> {
  // Create a new schema builder with the provided database connection
  let mut builder = Builder::new(&CONTEXT, database.clone());

  // Register the entities
  seaography::register_entities!(builder, [usersEntities]);

  // Register the active enums
  builder.register_enumeration::<users::enums::UserStatus>();
  builder.register_enumeration::<users::enums::UserRole>();

  // Register the custom scalars
  builder
    .set_depth_limit(depth)
    .set_complexity_limit(complexity)
    .schema_builder()
    .data(database)
    .finish()
}
