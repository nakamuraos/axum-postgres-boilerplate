use async_graphql::dynamic::ResolverContext;
use seaography::GuardsConfig;

use crate::modules::users::enums::UserRole;

pub fn admin_guard(ctx: &ResolverContext) -> seaography::GuardAction {
  // Get the user role from the context
  tracing::info!("Context data: {:?}", ctx.data::<UserRole>());
  if let Some(role) = ctx.data_opt::<UserRole>() {
    if *role == UserRole::Admin {
      return seaography::GuardAction::Allow;
    }
  }
  seaography::GuardAction::Block(Some("Admin role required".to_string()))
}

pub fn setup_guards() -> GuardsConfig {
  tracing::info!("Setting up GraphQL guards");
  let mut config = GuardsConfig::default();

  // Add entity guards
  config
    .entity_guards
    .insert("users".to_string(), Box::new(admin_guard));
  tracing::info!("Added entity guard for 'users'");

  // Add field guards for specific fields that require admin access
  config
    .field_guards
    .insert("users.role".to_string(), Box::new(admin_guard));
  config
    .field_guards
    .insert("users.status".to_string(), Box::new(admin_guard));
  tracing::info!("Added field guards for 'users.role' and 'users.status'");

  config
}
