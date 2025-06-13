use server::common::cfg::Configuration;
use server::common::telemetry;
use server::database::Db;
use tokio::net::TcpListener;
use tokio::signal;

#[tokio::main]
async fn main() {
  // Loads the .env file located in the environment's current directory or its parents in sequence.
  // .env used only for development, so we discard error in all other cases.
  dotenvy::dotenv().ok();

  // Tries to load tracing config from environment (RUST_LOG) or uses "debug".
  telemetry::setup_tracing();

  // Parse configuration from the environment.
  // This will exit with a help message if something is wrong.
  tracing::debug!("Initializing configuration");
  let cfg = Configuration::new();

  // Initialize db connection.
  tracing::debug!("Initializing db connection");
  let db = Db::new(&cfg).await.expect("Failed to initialize db");

  tracing::debug!("Running migrations");
  db.run_migrations().await.expect("Failed to run migrations");

  // Spin up our server.
  tracing::info!("Starting server on {}", cfg.listen_address);
  let listener = TcpListener::bind(&cfg.listen_address)
    .await
    .expect("Failed to bind address");

  let router = server::app::router(cfg.clone(), db);

  tracing::info!("Swagger at http://{}{}", cfg.listen_address, "/docs");
  tracing::info!(
    "GraphQL at http://{}{}",
    cfg.listen_address,
    cfg.graphql_endpoint
  );

  axum::serve(listener, router)
    .with_graceful_shutdown(shutdown_signal())
    .await
    .expect("Failed to start server")
}

async fn shutdown_signal() {
  signal::ctrl_c()
    .await
    .expect("Failed to listen for shutdown signal");
  println!("Shutdown signal received. Shutting down...");
}
