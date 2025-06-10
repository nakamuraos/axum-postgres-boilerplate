use serde::Deserialize;
use std::{
  net::{Ipv6Addr, SocketAddr},
  str::FromStr,
  sync::Arc,
};
use tracing::info;

pub type Config = Arc<Configuration>;

#[derive(Deserialize, Debug)]
pub struct Configuration {
  /// The environment in which to run the application.
  pub env: Environment,

  /// The address to listen on.
  pub listen_address: SocketAddr,

  /// The port to listen on.
  pub app_port: u16,

  /// The graphql endpoint
  pub graphql_endpoint: String,

  /// The DSN for the database. Currently, only PostgreSQL is supported.
  pub db_dsn: String,

  /// Maximum number of connections in the database pool
  pub db_pool_max_size: u32,

  /// Database connection timeout in seconds
  pub db_timeout: u64,
}

#[derive(Deserialize, Debug)]
pub enum Environment {
  Development,
  Production,
}

impl Configuration {
  /// Creates a new configuration from environment variables.
  pub fn new() -> Config {
    let env = env_var("APP_ENVIRONMENT")
            .parse::<Environment>()
            .expect("Unable to parse the value of the APP_ENVIRONMENT environment variable. Please make sure it is either \"development\" or \"production\".");

    let app_port = env_var("PORT")
            .parse::<u16>()
            .expect("Unable to parse the value of the PORT environment variable. Please make sure it is a valid unsigned 16-bit integer");

    // Graphql endpoint
    let graphql_endpoint =
      std::env::var("GRAPHQL_ENDPOINT").unwrap_or_else(|_| "/graphql".to_string());

    let db_dsn = env_var("DATABASE_URL");

    // Default pool size is 10 if not specified
    let db_pool_max_size = std::env::var("DATABASE_POOL_MAX_SIZE")
            .unwrap_or_else(|_| "10".to_string())
            .parse::<u32>()
            .expect("Unable to parse the value of the DATABASE_POOL_MAX_SIZE environment variable. Please make sure it is a valid unsigned 32-bit integer");

    // Default timeout is 5 seconds if not specified
    let db_timeout = std::env::var("DATABASE_TIMEOUT")
            .unwrap_or_else(|_| "5".to_string())
            .parse::<u64>()
            .expect("Unable to parse the value of the DATABASE_TIMEOUT environment variable. Please make sure it is a valid unsigned 64-bit integer");

    let listen_address = SocketAddr::from((Ipv6Addr::UNSPECIFIED, app_port));

    let config = Arc::new(Configuration {
      env,
      listen_address,
      app_port,
      graphql_endpoint,
      db_dsn,
      db_pool_max_size,
      db_timeout,
    });

    // Log the current configuration
    info!(?config, "Application configuration loaded");

    config
  }

  /// Sets the database DSN.
  /// This method is used in tests to override the database DSN.
  pub fn set_dsn(&mut self, db_dsn: String) {
    self.db_dsn = db_dsn
  }
}

impl FromStr for Environment {
  type Err = String;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "development" => Ok(Environment::Development),
      "production" => Ok(Environment::Production),
      _ => Err(format!(
        "Invalid environment: {}. Please make sure it is either \"development\" or \"production\".",
        s
      )),
    }
  }
}

pub fn env_var(name: &str) -> String {
  std::env::var(name)
    .map_err(|e| format!("{}: {}", name, e))
    .expect("Missing environment variable")
}
