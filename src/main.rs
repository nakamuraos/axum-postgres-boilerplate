use async_graphql::{dynamic, http::GraphiQLSource};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};

use axum::{response::Html, routing::get, Router};
use server::common::cfg::Configuration;
use server::common::telemetry;
use server::database::Db;
use tokio::net::TcpListener;
use tokio::signal;

mod query_root;

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

  let conn = db.conn.clone();
  let router = server::router(cfg.clone(), db);
  let schema = query_root::schema(conn, None, None).unwrap();

  let app = Router::new()
    .route("/graphql", get(graphql_playground).post(graphql_handler))
    .with_state(schema.clone())
    .merge(router);

  tracing::info!("Swagger at http://{}{}", cfg.listen_address, "/docs");
  tracing::info!("GraphQL at http://{}{}", cfg.listen_address, cfg.graphql_endpoint);

  axum::serve(listener, app)
    .with_graceful_shutdown(shutdown_signal())
    .await
    .expect("Failed to start server")
}

async fn graphql_handler(
  schema: axum::extract::State<dynamic::Schema>,
  req: GraphQLRequest,
) -> GraphQLResponse {
  schema.execute(req.into_inner()).await.into()
}

async fn graphql_playground() -> Html<String> {
  Html(GraphiQLSource::build().endpoint("/graphql").finish())
}

async fn shutdown_signal() {
  signal::ctrl_c()
    .await
    .expect("Failed to listen for shutdown signal");
  println!("Shutdown signal received. Shutting down...");
}
